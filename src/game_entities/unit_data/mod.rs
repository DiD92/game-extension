use crate::database::{
    DbConnector,
    inventory::{EntryUses, InventoryId, SlotType},
    kit::KitId,
    role::RoleId,
    skill::SkillId,
    unit::{UnitEntry, UnitId},
};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

mod godot_api;
mod initializers;
mod inventory;

pub(crate) use inventory::*;

pub(crate) type UnitIdx = u32;
pub(crate) type InventoryIdx = u32;

#[derive(GodotClass, Default, Clone)]
#[class(no_init, base=RefCounted)]
pub(crate) struct UnitData {
    // Identifiers
    #[export]
    unit_id: UnitId,
    #[export]
    unit_idx: UnitIdx,
    // Unit display data
    #[export]
    display_name: StringName,
    #[export]
    display_description: StringName,
    #[export]
    avatar_id: StringName,
    #[export]
    sprite_id: StringName,
    // Unit display stats
    #[export]
    level: u8,
    #[export]
    experience: u8,
    // Unit combat data - Stat Basis
    #[export]
    base_htp: u8,
    #[export]
    base_str: u8,
    #[export]
    base_mag: u8,
    #[export]
    base_def: u8,
    #[export]
    base_spt: u8,
    #[export]
    base_agi: u8,
    #[export]
    base_dex: u8,
    #[export]
    base_mov: u8,
    // Unit combat data - Stat Modifiers
    #[export]
    mod_htp: i8,
    #[export]
    mod_str: i8,
    #[export]
    mod_mag: i8,
    #[export]
    mod_def: i8,
    #[export]
    mod_spt: i8,
    #[export]
    mod_agi: i8,
    #[export]
    mod_dex: i8,
    #[export]
    mod_mov: i8,
    // Unit combat data - Consumable Stats
    #[export]
    current_htp: u8,
    // Unit combat data - Role/Kit data
    #[export]
    active_role_id: RoleId,
    #[export]
    active_kit_id: KitId,
    // Unit combat data - Skill data
    personal_skill_id: Option<SkillId>,
    equipped_skill_ids: Vec<SkillId>,
    // Unit combat data - Inventory
    // Array for ease of indexing
    inventory_slots: [InventorySlot; 6],
    // Current slot limits for eequipped kit
    #[export]
    max_physical_slots: u8,
    #[export]
    max_magical_slots: u8,
    #[export]
    max_support_slots: u8,
    #[export]
    max_item_slots: u8,
    // Index of the equipment_slot equipped
    #[export]
    equipped_slot_idx: i8,
    // Unit combat data - Interaction ranges
    #[export]
    attack_range: Vector2i,
    #[export]
    support_range: Vector2i,
}

impl UnitData {
    fn try_recompute_ranges(&mut self, db_link: GdRef<DbConnector>) -> bool {
        let mut maybe_attack_range = None::<Vector2i>;
        let mut maybe_support_range = None::<Vector2i>;

        for (slot_type, slot_entry) in self
            .inventory_slots
            .iter()
            .filter(|entry| entry.slot_entry.is_some())
            .map(|entry| (entry.slot_type, entry.slot_entry.as_ref().unwrap()))
        {
            let db_entry = db_link.inventory.get(&slot_entry.id);
            if db_entry.is_none() {
                godot_error!("Equipment data not found for [{}]", &slot_entry.id);
                return false;
            }
            let db_entry = db_entry.unwrap();

            if let Some(range) = db_entry._variant.get_range() {
                let entry_slot_type = db_entry._variant.get_slot_type();
                if slot_type != entry_slot_type {
                    godot_error!(
                        "Mismatched slot types [{:?}] - [{:?}]",
                        slot_type,
                        entry_slot_type
                    );
                    return false;
                }

                let maybe_target = match slot_type {
                    SlotType::Physical | SlotType::Magical => &mut maybe_attack_range,
                    SlotType::Support => &mut maybe_support_range,
                    SlotType::Item => {
                        godot_error!(
                            "Found item while iterating equipment slots [{}]",
                            &slot_entry.id
                        );
                        return false;
                    }
                };

                if let Some(target) = maybe_target {
                    target.x = std::cmp::min(target.x, range.x);
                    target.y = std::cmp::max(target.y, range.y);
                } else {
                    *maybe_target = Some(range);
                }
            }
        }

        self.attack_range = maybe_attack_range.unwrap_or_default();
        self.support_range = maybe_support_range.unwrap_or_default();

        true
    }

    fn recompute_equipped_slot(&mut self) {
        self.equipped_slot_idx = -1;

        if let Some(slot_idx) = self
            .inventory_slots
            .iter()
            .position(|slot| slot.contains_equipment() && !slot.is_empty())
        {
            self.equipped_slot_idx = slot_idx as i8;
        }
    }
}
