use super::dialogue::DialogueState;
use crate::{
    database::{
        army::ArmyId, chapter::Vector2u8, effect::EffectId, personality::PersonalityId,
        unit::UnitId,
    },
    game_entities::unit_data::UnitIdx,
};

use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod preparation;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct UnitState {
    army_id: ArmyId,
    unit_id: UnitId,
    unit_idx: UnitIdx,
    current_htp: u8,
    has_acted: bool,
    // TODO: Implement
    effects_queue: Vec<EffectId>,
}

impl GodotConvert for UnitState {
    type Via = Dictionary;
}

impl ToGodot for UnitState {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut unit_state_dict = Dictionary::new();

        unit_state_dict.set("army_id", self.army_id.clone());
        unit_state_dict.set("unit_id", self.unit_id.clone());
        unit_state_dict.set("unit_idx", self.unit_idx);
        unit_state_dict.set("current_htp", self.current_htp);
        unit_state_dict.set("has_acted", self.has_acted);
        unit_state_dict.set(
            "effects_queue",
            self.effects_queue
                .iter()
                .map(|eff| eff.to_variant())
                .collect::<VariantArray>(),
        );

        unit_state_dict
    }
}

impl FromGodot for UnitState {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            army_id: ArmyId::from_variant(&via.at("army_id")),
            unit_id: UnitId::from_variant(&via.at("unit_id")),
            unit_idx: UnitIdx::from_variant(&via.at("unit_idx")),
            current_htp: u8::from_variant(&via.at("current_htp")),
            has_acted: bool::from_variant(&via.at("has_acted")),
            effects_queue: VariantArray::from_variant(&via.at("effects_queue"))
                .iter_shared()
                .map(|item| EffectId::from_variant(&item))
                .collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub(super) struct ArmyState {
    army_id: ArmyId,
    army_state: u8,
}

impl GodotConvert for ArmyState {
    type Via = Dictionary;
}

impl ToGodot for ArmyState {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut flag_dict = Dictionary::new();

        flag_dict.set("army_id", self.army_id.clone());
        flag_dict.set("army_state", self.army_state);

        flag_dict
    }
}

impl FromGodot for ArmyState {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            army_id: ArmyId::from_variant(&via.at("army_id")),
            army_state: u8::from_variant(&via.at("army_state")),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct DefeatedUnit {
    army_id: ArmyId,
    unit_id: UnitId,
    unit_idx: UnitIdx,
    defeated_at: Vector2u8,
}

impl GodotConvert for DefeatedUnit {
    type Via = Dictionary;
}

impl ToGodot for DefeatedUnit {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut flag_dict = Dictionary::new();

        flag_dict.set("army_id", self.army_id.clone());
        flag_dict.set("unit_id", self.unit_id.clone());
        flag_dict.set("unit_idx", self.unit_idx);
        flag_dict.set("defeated_at", self.defeated_at.to_godot());

        flag_dict
    }
}

impl FromGodot for DefeatedUnit {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            army_id: ArmyId::from_variant(&via.at("army_id")),
            unit_id: UnitId::from_variant(&via.at("unit_id")),
            unit_idx: UnitIdx::from_variant(&via.at("unit_idx")),
            defeated_at: Vector2u8::from_variant(&via.at("defeated_at")),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct BattleState {
    /// Optional preparations sub-segment
    preparations_state: Option<preparation::PreparationsState>,
    /// Optional dialogue before battle start sub-segment
    pre_dialogue_state: Option<DialogueState>,
    battle_active: bool,
    current_phase_idx: u8,
    pub(super) current_turn: u8,
    // Value is bitflags
    army_states: Vec<ArmyState>,
    cursor_position: Vector2u8,
    active_units: HashMap<Vector2u8, UnitState>,
    defeated_units: Vec<DefeatedUnit>,
    /// Table of unit idx to its personality identifier
    unit_personalities: HashMap<UnitIdx, PersonalityId>,
    /// Table of unit idx to its defend cell position
    /// this will only be used for units that have the
    /// MovementBehaviour::Defend as one of their possible
    /// personality entries
    /// If a unit has multiple MovementBehaviour::Defend
    /// personality entries, they will all share the
    /// same defend cell
    defend_cells: HashMap<UnitIdx, Vector2u8>,
}

impl GodotConvert for BattleState {
    type Via = Dictionary;
}

impl ToGodot for BattleState {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut dict = Dictionary::new();

        dict.set(
            "preparations_state",
            self.preparations_state
                .as_ref()
                .map(|p| p.to_variant())
                .unwrap_or(Dictionary::default().to_variant()),
        );
        dict.set(
            "pre_dialogue_state",
            self.pre_dialogue_state
                .as_ref()
                .map(|d| d.to_variant())
                .unwrap_or(Dictionary::default().to_variant()),
        );

        dict.set("battle_active", self.battle_active);
        dict.set("current_phase_idx", self.current_phase_idx);
        dict.set("current_turn", self.current_turn);
        dict.set(
            "army_states",
            self.army_states
                .iter()
                .map(|state| state.to_variant())
                .collect::<VariantArray>(),
        );
        dict.set("cursor_position", self.cursor_position.to_godot());
        dict.set(
            "active_units",
            self.active_units
                .iter()
                .map(|(pos, state)| (pos.to_variant(), state.to_variant()))
                .collect::<Dictionary>(),
        );
        dict.set("defeated_units", {
            self.defeated_units
                .iter()
                .map(|unit| unit.to_variant())
                .collect::<VariantArray>()
        });

        dict.set("unit_personalities", {
            self.unit_personalities
                .iter()
                .map(|(idx, id)| (idx.to_variant(), id.to_variant()))
                .collect::<Dictionary>()
        });

        dict.set("defend_cells", {
            self.defend_cells
                .iter()
                .map(|(idx, cell)| (idx.to_variant(), cell.to_variant()))
                .collect::<Dictionary>()
        });

        dict
    }
}

impl FromGodot for BattleState {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            preparations_state: via
                .get("preparations_state")
                .filter(|data| !Dictionary::from_variant(data).is_empty())
                .map(|data| preparation::PreparationsState::from_variant(&data)),
            pre_dialogue_state: via
                .get("pre_dialogue_state")
                .filter(|data| !Dictionary::from_variant(data).is_empty())
                .map(|data| DialogueState::from_variant(&data)),
            battle_active: bool::from_variant(&via.at("battle_active")),
            current_phase_idx: u8::from_variant(&via.at("current_phase_idx")),
            current_turn: u8::from_variant(&via.at("current_turn")),
            army_states: VariantArray::from_variant(&via.at("army_states"))
                .iter_shared()
                .map(|army_state| ArmyState::from_variant(&army_state))
                .collect(),
            cursor_position: Vector2u8::from_variant(&via.at("cursor_position")),
            active_units: Dictionary::from_variant(&via.at("active_units"))
                .iter_shared()
                .map(|(pos, state)| {
                    (
                        Vector2u8::from_variant(&pos),
                        UnitState::from_variant(&state),
                    )
                })
                .collect(),
            defeated_units: VariantArray::from_variant(&via.at("defeated_units"))
                .iter_shared()
                .map(|unit| DefeatedUnit::from_variant(&unit))
                .collect(),
            unit_personalities: Dictionary::from_variant(&via.at("unit_personalities"))
                .iter_shared()
                .map(|(idx, id)| {
                    (
                        UnitIdx::from_variant(&idx),
                        PersonalityId::from_variant(&id),
                    )
                })
                .collect(),
            defend_cells: Dictionary::from_variant(&via.at("defend_cells"))
                .iter_shared()
                .map(|(idx, cell)| (UnitIdx::from_variant(&idx), Vector2u8::from_variant(&cell)))
                .collect(),
        }
    }
}
