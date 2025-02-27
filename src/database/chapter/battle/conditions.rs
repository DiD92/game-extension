use super::{ArmyId, ToVariantArray, UnitId, Vector2u8};

use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
#[serde(tag = "type", content = "params")]
pub(crate) enum VictoryCondition {
    RoutArmies(Vec<ArmyId>),
    DefeatUnits(HashMap<ArmyId, Vec<UnitId>>),
    DefendUntil {
        defend_cells: Vec<Vector2u8>,
        until_turn: u8,
    },
    DefendRout {
        defend_cells: Vec<Vector2u8>,
        rout: Vec<ArmyId>,
    },
    ReachWithUnits {
        reach_cells: Vec<Vector2u8>,
        reach_with: Vec<UnitId>,
    },
}

impl GodotConvert for VictoryCondition {
    type Via = Dictionary;
}

impl ToGodot for VictoryCondition {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        match self {
            VictoryCondition::RoutArmies(army_list) => {
                dict! {
                    "type": 0_u8,
                    "params": army_list.to_variant_array()
                }
            }
            VictoryCondition::DefeatUnits(unit_map) => {
                let army_map = unit_map
                    .iter()
                    .map(|(army_id, unit_vec)| (army_id.clone(), unit_vec.to_variant_array()))
                    .collect::<Dictionary>();
                dict! {
                    "type": 1_u8,
                    "params": army_map
                }
            }
            VictoryCondition::DefendUntil {
                defend_cells,
                until_turn,
            } => {
                dict! {
                    "type": 2_u8,
                    "params": dict! {
                        "cells": defend_cells.to_variant_array(),
                        "until": *until_turn
                    }
                }
            }
            VictoryCondition::DefendRout { defend_cells, rout } => {
                dict! {
                    "type": 3_u8,
                    "params": dict! {
                        "cells": defend_cells.to_variant_array(),
                        "rout": rout.to_variant_array()
                    }
                }
            }
            VictoryCondition::ReachWithUnits {
                reach_cells,
                reach_with,
            } => {
                dict! {
                    "type": 4_u8,
                    "params": dict! {
                        "cells": reach_cells.to_variant_array(),
                        "with": reach_with.to_variant_array()
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
#[serde(tag = "type", content = "params")]
pub(crate) enum DefeatCondition {
    UnitsDefeated(Vec<UnitId>),
    ArmiesRouted(Vec<ArmyId>),
    TurnReached(u8),
}

impl GodotConvert for DefeatCondition {
    type Via = Dictionary;
}

impl ToGodot for DefeatCondition {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        match self {
            DefeatCondition::UnitsDefeated(units) => {
                dict! {
                    "type": 0_u8,
                    "params": units.to_variant_array()
                }
            }
            DefeatCondition::ArmiesRouted(armies) => {
                dict! {
                    "type": 1_u8,
                    "params": armies.to_variant_array()
                }
            }
            DefeatCondition::TurnReached(turn) => {
                dict! {
                    "type": 2_u8,
                    "params": *turn
                }
            }
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::{DefeatCondition, VictoryCondition};
    use crate::database::{
        DbConnector,
        chapter::{ChapterKey, battle::BattleConfig},
    };

    use godot::global::godot_error;
    use std::collections::HashSet;

    impl VictoryCondition {
        pub(crate) fn validate(
            &self,
            parent: &BattleConfig,
            segment_idx: usize,
            chapter_key: &ChapterKey,
            _db: &DbConnector,
        ) -> bool {
            match self {
                VictoryCondition::RoutArmies(armies) => armies.iter().all(|army_id| {
                    if !parent.enemy_armies.contains(army_id) {
                        godot_error!(
                            "[{}][{}] Could not find RoutArmies army_id in enemy_armies [{}]!",
                            chapter_key,
                            segment_idx,
                            army_id
                        );
                        false
                    } else {
                        true
                    }
                }),
                VictoryCondition::DefeatUnits(defeat_map) => {
                    for (army_id, defeat_units) in defeat_map.iter() {
                        if !parent.enemy_armies.contains(army_id) {
                            godot_error!(
                                "[{}][{}] Could not find DefeatUnits army in enemy_armies [{}]!",
                                chapter_key,
                                segment_idx,
                                army_id
                            );
                            return false;
                        }

                        if let Some(placements) = parent.unit_placements.get(army_id) {
                            for unit_id in defeat_units.iter() {
                                if !placements
                                    .iter()
                                    .any(|placement| &placement.unit_id == unit_id)
                                {
                                    godot_error!(
                                        "[{}][{}] Could not find DefeatUnits unit_id in placements [{}][{}]!",
                                        chapter_key,
                                        segment_idx,
                                        army_id,
                                        unit_id
                                    );
                                    return false;
                                }
                            }
                        } else {
                            godot_error!(
                                "[{}][{}] Could not find DefeatUnits placements [{}]!",
                                chapter_key,
                                segment_idx,
                                army_id
                            );
                            return false;
                        }
                    }
                    true
                }
                VictoryCondition::DefendUntil {
                    defend_cells,
                    until_turn,
                } => {
                    let defend_unique_count =
                        defend_cells.iter().cloned().collect::<HashSet<_>>().len();
                    if defend_unique_count != defend_cells.len() {
                        godot_error!(
                            "[{}][{}] Repeated cells in DefendUntil!",
                            chapter_key,
                            segment_idx
                        );
                        return false;
                    }

                    if *until_turn < 5 {
                        godot_error!(
                            "[{}][{}] DefendUntil turn should be 5 or higher!",
                            chapter_key,
                            segment_idx
                        );
                        return false;
                    }

                    true
                }
                VictoryCondition::DefendRout { defend_cells, rout } => {
                    let defend_unique_count =
                        defend_cells.iter().cloned().collect::<HashSet<_>>().len();
                    if defend_unique_count != defend_cells.len() {
                        godot_error!(
                            "[{}][{}] Repeated cells in DefendRout!",
                            chapter_key,
                            segment_idx
                        );
                        return false;
                    }

                    for army_id in rout.iter() {
                        if !parent.enemy_armies.contains(army_id) {
                            godot_error!(
                                "[{}][{}] Could not find DefendRout army_id in enemy_armies [{}]!",
                                chapter_key,
                                segment_idx,
                                army_id
                            );
                            return false;
                        }
                    }

                    true
                }
                VictoryCondition::ReachWithUnits {
                    reach_cells,
                    reach_with,
                } => {
                    let reach_unique_count =
                        reach_cells.iter().cloned().collect::<HashSet<_>>().len();
                    if reach_unique_count != reach_cells.len() {
                        godot_error!(
                            "[{}][{}] Repeated cells in ReachWithUnits!",
                            chapter_key,
                            segment_idx
                        );
                        return false;
                    }

                    if let Some(placements) = parent.unit_placements.get(&parent.player_army) {
                        for unit_id in reach_with.iter() {
                            if !placements
                                .iter()
                                .any(|placement| &placement.unit_id == unit_id)
                            {
                                godot_error!(
                                    "[{}][{}] Could not find ReachWithUnits unit_id in placements [{}][{}]!",
                                    chapter_key,
                                    segment_idx,
                                    &parent.player_army,
                                    unit_id
                                );
                                return false;
                            }
                        }
                    } else {
                        godot_error!(
                            "[{}][{}] Could not find ReachWithUnits placements [{}]!",
                            chapter_key,
                            segment_idx,
                            &parent.player_army
                        );
                        return false;
                    }

                    true
                }
            }
        }
    }

    impl DefeatCondition {
        pub(crate) fn validate(
            &self,
            parent: &BattleConfig,
            segment_idx: usize,
            chapter_key: &ChapterKey,
            _db: &DbConnector,
        ) -> bool {
            match self {
                DefeatCondition::UnitsDefeated(player_units) => {
                    if let Some(placements) = parent.unit_placements.get(&parent.player_army) {
                        for unit_id in player_units.iter() {
                            if !placements
                                .iter()
                                .any(|placement| &placement.unit_id == unit_id)
                            {
                                godot_error!(
                                    "[{}][{}] Could not find UnitsDefeated unit_id in placements [{}][{}]!",
                                    chapter_key,
                                    segment_idx,
                                    &parent.player_army,
                                    unit_id
                                );
                                return false;
                            }
                        }

                        true
                    } else {
                        godot_error!(
                            "[{}][{}] Could not find ReachWithUnits placements [{}]!",
                            chapter_key,
                            segment_idx,
                            &parent.player_army
                        );
                        false
                    }
                }
                DefeatCondition::ArmiesRouted(armies) => {
                    for army_id in armies.iter() {
                        if army_id != &parent.player_army && !parent.allied_armies.contains(army_id)
                        {
                            godot_error!(
                                "[{}][{}] Could not find ArmiesRouted army_id in player or allied armies [{}]!",
                                chapter_key,
                                segment_idx,
                                &army_id
                            );
                            return false;
                        }
                    }

                    true
                }
                DefeatCondition::TurnReached(turn) => {
                    if *turn < 5 {
                        godot_error!(
                            "[{}][{}] TurnReached turn should be 5 or higher!",
                            chapter_key,
                            segment_idx
                        );
                        return false;
                    }

                    true
                }
            }
        }
    }
}
