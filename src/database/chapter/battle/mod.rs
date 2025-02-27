use super::{DbId, dialogue::DialogueConfig};
use crate::{
    database::{army::ArmyId, personality::PersonalityId, unit::UnitId},
    traits::{ToDefaultVariant, ToVariantArray},
};

use godot::prelude::*;
use serde::{Deserialize, Serialize, de::Visitor};
use std::{collections::HashMap, fmt::Display};

mod conditions;
mod preparation;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub(crate) struct Vector2u8 {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

impl Serialize for Vector2u8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let position_str = format!("{}:{}", self.x, self.y);

        serializer.serialize_str(&position_str)
    }
}

impl Display for Vector2u8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}:{})", self.x, self.y)
    }
}

struct CellPositionVisitor;

impl Visitor<'_> for CellPositionVisitor {
    type Value = Vector2u8;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an str with two numbers separated by a ':' character")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let str_split = v
            .split(':')
            .map(str::parse::<u8>)
            .collect::<Result<Vec<_>, _>>();

        match str_split {
            Ok(entries) if entries.len() == 2 => Ok(Self::Value {
                x: entries[0],
                y: entries[1],
            }),
            _ => Err(E::custom("Failed to deserialize CellPosition")),
        }
    }
}

impl<'de> Deserialize<'de> for Vector2u8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(CellPositionVisitor)
    }
}

impl GodotConvert for Vector2u8 {
    type Via = Vector2i;
}

impl ToGodot for Vector2u8 {
    type ToVia<'v> = Vector2i;

    fn to_godot(&self) -> Self::Via {
        Self::Via {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl FromGodot for Vector2u8 {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            x: via.x as u8,
            y: via.y as u8,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PlacementDetails {
    unit_id: UnitId,
    positions: Vec<Vector2u8>,
    personality: PersonalityId,
}

impl GodotConvert for PlacementDetails {
    type Via = Dictionary;
}

impl ToGodot for PlacementDetails {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "unit_id": self.unit_id.clone(),
            "positions": self.positions.to_variant_array(),
            "personality": self.personality.clone(),
        }
    }
}

pub(crate) type MapKey = DbId;

pub(crate) type UnitPlacements = Vec<PlacementDetails>;
pub(crate) type ArmyPlacements = HashMap<ArmyId, UnitPlacements>;

const fn default_restore_health() -> bool {
    true
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct BattleConfig {
    map_key: MapKey,
    #[serde(default)]
    preparations: Option<preparation::PreparationConfig>,
    #[serde(default)]
    pre_dialogue: Option<DialogueConfig>,
    music_id: StringName,
    #[serde(default = "default_restore_health")]
    restore_health: bool,
    player_army: ArmyId,
    enemy_armies: Vec<ArmyId>,
    #[serde(default)]
    allied_armies: Vec<ArmyId>,
    active_armies: Vec<ArmyId>,
    starting_army: ArmyId,
    victory_condition: conditions::VictoryCondition,
    defeat_condition: conditions::DefeatCondition,
    cursor_start: Vector2u8,
    unit_placements: ArmyPlacements,
}

impl GodotConvert for BattleConfig {
    type Via = Dictionary;
}

impl ToGodot for BattleConfig {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "map_key": self.map_key.clone(),
            "preparations": self.preparations.to_default_variant(),
            "pre_dialogue": self.pre_dialogue.to_default_variant(),
            "music_id": self.music_id.clone(),
            "restore_health": self.restore_health,
            "player_army": self.player_army.clone(),
            "enemy_armies": self.enemy_armies.to_variant_array(),
            "allied_armies": self.allied_armies.to_variant_array(),
            "active_armies": self.active_armies.to_variant_array(),
            "starting_army": self.starting_army.clone(),
            "victory_condition": self.victory_condition.to_variant(),
            "defeat_condition": self.defeat_condition.to_variant(),
            "cursor_start": self.cursor_start.to_variant(),
            "unit_placements": self.unit_placements
                .iter()
                .map(|(k, v)| (k.to_variant(), v.to_variant_array()))
                .collect::<Dictionary>(),
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::{BattleConfig, Vector2u8};
    use crate::database::{DbConnector, chapter::ChapterKey};

    use godot::{classes::ResourceLoader, global::godot_error};
    use std::collections::HashSet;

    const BASE_BATTLE_MAP_PATH: &str = "res://scenes/maps/";
    const BASE_BATTLE_MUSIC_PATH: &str = "res://assets/sound/music/";

    impl BattleConfig {
        pub(crate) fn validate(
            &self,
            segment_idx: usize,
            chapter_key: &ChapterKey,
            db: &DbConnector,
        ) -> bool {
            let map_path = format!("{}{}.tscn", BASE_BATTLE_MAP_PATH, self.map_key);
            if !ResourceLoader::singleton().exists(&map_path) {
                godot_error!(
                    "[{}][{}] Could not find map [{}]",
                    chapter_key,
                    segment_idx,
                    map_path
                );
                return false;
            }

            if let Some(ref preparations) = self.preparations {
                if !preparations.validate(self, segment_idx, chapter_key, db) {
                    return false;
                }
            }

            if let Some(ref dialogue) = self.pre_dialogue {
                if !dialogue.validate(segment_idx, chapter_key, db) {
                    godot_error!(
                        "[{}][{}] Pre-battle dialogue validation failed",
                        chapter_key,
                        segment_idx
                    );
                    return false;
                }
            }

            let battle_music_path = format!("{}{}.wav", BASE_BATTLE_MUSIC_PATH, self.music_id);
            if !ResourceLoader::singleton().exists(&battle_music_path) {
                godot_error!(
                    "[{}][{}] Could not find music [{}]",
                    chapter_key,
                    segment_idx,
                    battle_music_path
                );
                return false;
            }

            if !self.validate_armies(segment_idx, chapter_key, db) {
                return false;
            }

            if !self
                .victory_condition
                .validate(self, segment_idx, chapter_key, db)
            {
                return false;
            }

            if !self
                .defeat_condition
                .validate(self, segment_idx, chapter_key, db)
            {
                return false;
            }

            if !self.validate_unit_placements(segment_idx, chapter_key, db) {
                return false;
            }

            true
        }

        fn validate_armies(
            &self,
            segment_idx: usize,
            chapter_key: &ChapterKey,
            db: &DbConnector,
        ) -> bool {
            // Player army checks
            if self.player_army.is_empty() || !db.armies.contains_key(&self.player_army) {
                godot_error!(
                    "[{}][{}] Could not find army [{}]",
                    chapter_key,
                    segment_idx,
                    self.player_army
                );
                return false;
            }

            if self.enemy_armies.contains(&self.player_army)
                || self.allied_armies.contains(&self.player_army)
            {
                godot_error!(
                    "[{}][{}] player_army found in enemy/allied armies!",
                    segment_idx,
                    chapter_key,
                );
                return false;
            }

            let unique_enemy_armies = self.enemy_armies.iter().cloned().collect::<HashSet<_>>();
            if self.enemy_armies.len() != unique_enemy_armies.len() {
                godot_error!(
                    "[{}][{}] Found repeated entries in enemy armies!",
                    chapter_key,
                    segment_idx
                );
                return false;
            }

            let unique_allied_armies = self.allied_armies.iter().cloned().collect::<HashSet<_>>();
            if self.allied_armies.len() != unique_allied_armies.len() {
                godot_error!(
                    "[{}][{}] Found repeated entries in allied armies!",
                    chapter_key,
                    segment_idx
                );
                return false;
            }

            if !self.active_armies.contains(&self.player_army) {
                godot_error!(
                    "[{}][{}] player_army not found in active armies!",
                    chapter_key,
                    segment_idx
                );
                return false;
            }

            // Enemy armies checks
            if self.enemy_armies.is_empty() {
                godot_error!(
                    "[{}][{}] Enemy armies cannot be empty",
                    chapter_key,
                    segment_idx
                );
                return false;
            }

            for army_id in self.enemy_armies.iter() {
                if !db.armies.contains_key(army_id) {
                    godot_error!(
                        "[{}][{}] Could not find enemy army [{}]",
                        chapter_key,
                        segment_idx,
                        army_id
                    );
                    return false;
                }

                if self.allied_armies.contains(army_id) {
                    godot_error!(
                        "[{}][{}] Enemy army [{}] found in allied armies!",
                        chapter_key,
                        segment_idx,
                        army_id
                    );
                    return false;
                }
            }

            let unique_enemy_armies = self.enemy_armies.iter().cloned().collect::<HashSet<_>>();
            if self.enemy_armies.len() != unique_enemy_armies.len() {
                godot_error!(
                    "[{}][{}] Found repeated entries in enemy armies!",
                    chapter_key,
                    segment_idx
                );
                return false;
            }

            // Allied armies checks
            for army_id in self.allied_armies.iter() {
                if !db.armies.contains_key(army_id) {
                    godot_error!(
                        "[{}][{}] Could not find allied army [{}]",
                        chapter_key,
                        segment_idx,
                        army_id
                    );
                    return false;
                }
            }

            let unique_allied_armies = self.allied_armies.iter().cloned().collect::<HashSet<_>>();
            if self.allied_armies.len() != unique_allied_armies.len() {
                godot_error!(
                    "[{}][{}] Found repeated entries in allied armies!",
                    chapter_key,
                    segment_idx
                );
                return false;
            }

            // Active armies checks
            if self.active_armies.is_empty() {
                godot_error!(
                    "[{}][{}] Active armies cannot be empty!",
                    chapter_key,
                    segment_idx,
                );
                return false;
            }

            let unique_active_armies = self.active_armies.iter().cloned().collect::<HashSet<_>>();
            if self.active_armies.len() != unique_active_armies.len() {
                godot_error!(
                    "[{}][{}] Found repeated entries in active armies!",
                    chapter_key,
                    segment_idx
                );
                return false;
            }

            // Starting army checks
            if self.starting_army.is_empty() || !db.armies.contains_key(&self.starting_army) {
                godot_error!(
                    "[{}][{}] Could not find starting army [{}]",
                    chapter_key,
                    segment_idx,
                    self.starting_army
                );
                return false;
            }

            if !self.active_armies.contains(&self.starting_army) {
                godot_error!(
                    "[{}][{}] Starting army not found in active armies!",
                    segment_idx,
                    chapter_key,
                );
                return false;
            }

            true
        }

        fn validate_unit_placements(
            &self,
            segment_idx: usize,
            chapter_key: &ChapterKey,
            db: &DbConnector,
        ) -> bool {
            let participant_armies = [&self.player_army]
                .into_iter()
                .chain(self.enemy_armies.iter())
                .chain(self.allied_armies.iter());

            let mut visited_cells = HashSet::<Vector2u8>::new();

            for army_id in participant_armies {
                if let Some(placements) = self.unit_placements.get(army_id) {
                    for placement in placements.iter() {
                        if !db.units.contains_key(&placement.unit_id) {
                            godot_error!(
                                "[{}][{}] Could not find unit_id [{}]!",
                                chapter_key,
                                segment_idx,
                                &placement.unit_id
                            );
                            return false;
                        }

                        for cell in placement.positions.iter() {
                            if visited_cells.contains(cell) {
                                godot_error!(
                                    "[{}][{}] Placement cell already used [{:?}]!",
                                    segment_idx,
                                    chapter_key,
                                    cell,
                                );
                                return false;
                            } else {
                                visited_cells.insert(*cell);
                            }
                        }

                        if !db.personalities.contains_key(&placement.personality) {
                            godot_error!(
                                "[{}][{}] Unit personality identifier [{}] not found in database!",
                                chapter_key,
                                segment_idx,
                                &placement.personality
                            );
                            return false;
                        }
                    }
                } else {
                    godot_error!(
                        "[{}][{}] Unit placement is missing army_id [{}]!",
                        chapter_key,
                        segment_idx,
                        army_id,
                    );
                    return false;
                }
            }

            true
        }
    }
}
