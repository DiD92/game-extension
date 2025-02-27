use super::{KitId, RoleId, SkillId, UnitId};
use crate::{
    game_entities::unit_data::{InventorySlot, UnitIdx},
    traits::ToVariantOption,
};

use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Not};

pub(super) type BarracksUnits = HashMap<UnitId, UnitSaveData>;

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct UnitSaveData {
    unit_id: UnitId,
    unit_idx: UnitIdx,
    level: u8,
    exp: u8,
    base_htp: u8,
    base_str: u8,
    base_mag: u8,
    base_def: u8,
    base_spt: u8,
    base_agi: u8,
    base_dex: u8,
    base_mov: u8,
    active_role: RoleId,
    active_kit: KitId,
    personal_skill: Option<SkillId>,
    skill_slots: Vec<SkillId>,
    equipped_slot_idx: i8,
    inventory_slots: [InventorySlot; 6],
}

impl GodotConvert for UnitSaveData {
    type Via = Dictionary;
}

impl ToGodot for UnitSaveData {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut entry_dict = Dictionary::new();

        entry_dict.set("unit_id", self.unit_id.clone());
        entry_dict.set("unit_idx", self.unit_idx);

        entry_dict.set("level", self.level);
        entry_dict.set("exp", self.exp);

        entry_dict.set("base_htp", self.base_htp);
        entry_dict.set("base_str", self.base_str);
        entry_dict.set("base_mag", self.base_mag);
        entry_dict.set("base_def", self.base_def);
        entry_dict.set("base_spt", self.base_spt);
        entry_dict.set("base_agi", self.base_agi);
        entry_dict.set("base_dex", self.base_dex);
        entry_dict.set("base_mov", self.base_mov);

        entry_dict.set("active_role", self.active_role.clone());
        entry_dict.set("active_kit", self.active_kit.clone());

        entry_dict.set("personal_skill", self.personal_skill.maybe_to_variant());

        entry_dict.set(
            "skill_slots",
            self.skill_slots
                .iter()
                .map(|skill_id| skill_id.to_variant())
                .collect::<VariantArray>(),
        );

        entry_dict.set("equipped_slot_idx", self.equipped_slot_idx.to_variant());
        entry_dict.set(
            "inventory_slots",
            self.inventory_slots
                .iter()
                .map(|slot| slot.to_variant())
                .collect::<VariantArray>(),
        );

        entry_dict
    }
}

impl FromGodot for UnitSaveData {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            unit_id: UnitId::from_variant(&via.at("unit_id")),
            unit_idx: UnitIdx::from_variant(&via.at("unit_idx")),
            level: u8::from_variant(&via.at("level")),
            exp: u8::from_variant(&via.at("exp")),
            base_htp: u8::from_variant(&via.at("base_htp")),
            base_str: u8::from_variant(&via.at("base_str")),
            base_mag: u8::from_variant(&via.at("base_mag")),
            base_def: u8::from_variant(&via.at("base_def")),
            base_spt: u8::from_variant(&via.at("base_spt")),
            base_agi: u8::from_variant(&via.at("base_agi")),
            base_dex: u8::from_variant(&via.at("base_dex")),
            base_mov: u8::from_variant(&via.at("base_mov")),
            active_role: RoleId::from_variant(&via.at("active_role")),
            active_kit: KitId::from_variant(&via.at("active_kit")),
            personal_skill: {
                let skill = SkillId::from_variant(&via.at("personal_skill"));

                skill.is_empty().not().then_some(skill)
            },
            skill_slots: VariantArray::from_variant(&via.at("skill_slots"))
                .iter_shared()
                .map(|skill_id| SkillId::from_variant(&skill_id))
                .collect(),
            equipped_slot_idx: i8::from_variant(&via.at("equipped_slot_idx")),
            inventory_slots: {
                let slots_arr = VariantArray::from_variant(&via.at("inventory_slots"));
                let slots_iter = slots_arr
                    .iter_shared()
                    .map(|slot_state| InventorySlot::from_variant(&slot_state))
                    .enumerate();

                let mut slot_states = core::array::from_fn(|_| InventorySlot::default());

                for (i, slot_state) in slots_iter {
                    slot_states[i] = slot_state;
                }

                slot_states
            },
        }
    }
}
