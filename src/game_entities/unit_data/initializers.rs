use super::inventory::SlotEntry;
use crate::{game_entities::index_store::IndexStore, traits::GetAs};

use super::*;

impl UnitData {
    #[inline]
    fn init_basic_data(&mut self, unit_id: UnitId, unit_idx: UnitIdx, unit_entry: &UnitEntry) {
        self.unit_id = unit_id;
        self.unit_idx = unit_idx;

        self.display_name = unit_entry._n.name.clone();
        self.display_description = unit_entry._n.description.clone();

        self.avatar_id = unit_entry.avatar_id.clone();
        self.sprite_id = unit_entry.sprite_id.clone();
    }

    #[inline]
    fn init_stats(&mut self, unit_entry: &UnitEntry, overrides: &Dictionary) {
        self.level = overrides.get_as("level", unit_entry.stats.level);
        self.experience = overrides.get_as("exp", unit_entry.stats.experience);

        self.base_htp = overrides.get_as("base_htp", unit_entry.stats.htp);
        self.base_str = overrides.get_as("base_str", unit_entry.stats.str);
        self.base_mag = overrides.get_as("base_mag", unit_entry.stats.mag);
        self.base_def = overrides.get_as("base_def", unit_entry.stats.def);
        self.base_spt = overrides.get_as("base_spt", unit_entry.stats.spt);
        self.base_agi = overrides.get_as("base_agi", unit_entry.stats.agi);
        self.base_dex = overrides.get_as("base_dex", unit_entry.stats.dex);
        self.base_mov = overrides.get_as("base_mov", unit_entry.stats.mov);

        self.current_htp = overrides.get_as("current_htp", unit_entry.stats.htp);
    }

    pub(super) fn init_from_database(
        unit_id: UnitId,
        unit_idx: UnitIdx,
        unit_entry: &UnitEntry,
        overrides: Dictionary,
        db_link: GdRef<DbConnector>,
        idx_gen: &mut GdMut<IndexStore>,
    ) -> Option<Gd<Self>> {
        let mut unit_data = Self::default();

        unit_data.init_basic_data(unit_id, unit_idx, unit_entry);

        unit_data.init_stats(unit_entry, &overrides);

        unit_data.active_role_id = unit_entry.role_id.clone();
        unit_data.active_kit_id = unit_entry.kit_id.clone();

        unit_data.personal_skill_id = unit_entry.personal_skill_id.clone();
        unit_data.equipped_skill_ids = unit_entry.equipped_skill_ids.clone();

        let maybe_kit_data = db_link.kits.get(&unit_data.active_kit_id);
        if maybe_kit_data.is_none() {
            godot_error!(
                "Failed to get kit data [{}] in unit initializer!",
                &unit_data.active_kit_id
            );
            return None;
        }
        let kit_data = maybe_kit_data.unwrap();

        unit_data.max_physical_slots = kit_data.weapon_slots;
        unit_data.max_magical_slots = kit_data.magic_slots;
        unit_data.max_support_slots = kit_data.support_slots;
        unit_data.max_item_slots = kit_data.item_slots;

        let slot_ids = (0..kit_data.weapon_slots)
            .map(|slot_idx| {
                (
                    SlotType::Physical,
                    unit_entry.physical_slots.get(slot_idx as usize),
                )
            })
            .chain((0..kit_data.magic_slots).map(|slot_idx| {
                (
                    SlotType::Magical,
                    unit_entry.magical_slots.get(slot_idx as usize),
                )
            }))
            .chain((0..kit_data.support_slots).map(|slot_idx| {
                (
                    SlotType::Support,
                    unit_entry.support_slots.get(slot_idx as usize),
                )
            }))
            .chain(
                (0..kit_data.item_slots)
                    .map(|slot_idx| (SlotType::Item, unit_entry.item_slots.get(slot_idx as usize))),
            );

        for (slot_idx, (slot_type, maybe_slot_id)) in slot_ids.enumerate() {
            let entry = if let Some(slot_id) = maybe_slot_id {
                if let Some(entry) = db_link.inventory.get(slot_id) {
                    Some(SlotEntry {
                        id: slot_id.clone(),
                        idx: idx_gen.next_item_idx(),
                        uses: entry.uses,
                    })
                } else {
                    godot_error!("Entry data not found for [{}]", slot_id);
                    return None;
                }
            } else {
                None
            };

            let slot = InventorySlot {
                slot_type,
                slot_entry: entry,
            };
            unit_data.inventory_slots[slot_idx] = slot;
        }

        unit_data.equipped_slot_idx = unit_entry.equipped_slot_idx.unwrap_or(-1);

        if unit_data.try_recompute_ranges(db_link) {
            Some(Gd::from_object(unit_data))
        } else {
            None
        }
    }

    pub(super) fn init_from_save(
        save_data: Dictionary,
        db_link: GdRef<DbConnector>,
        overrides: Dictionary,
    ) -> Option<Gd<Self>> {
        let mut unit_data = Self::default();

        let combined_data = {
            let mut dict = Dictionary::new();
            dict.extend_dictionary(&save_data, true);
            dict.extend_dictionary(&overrides, true);
            dict
        };

        if !save_data.contains_all_keys(&varray![&"unit_id", &"unit_idx"]) {
            godot_warn!("Save data is invalid!");
            return None;
        }

        let unit_id = UnitId::from_variant(&combined_data.at("unit_id"));
        let unit_idx = UnitIdx::from_variant(&combined_data.at("unit_idx"));

        let unit_entry = db_link
            .units
            .get(&unit_id)
            .expect("Failed to get unit in init_save");

        unit_data.init_basic_data(unit_id, unit_idx, unit_entry);
        unit_data.init_stats(unit_entry, &combined_data);

        unit_data.active_role_id = RoleId::from_variant(&combined_data.at("active_role"));
        unit_data.active_kit_id = KitId::from_variant(&combined_data.at("active_kit"));

        unit_data.personal_skill_id = {
            let base_string = SkillId::from_variant(&combined_data.at("personal_skill"));
            if base_string.is_empty() {
                None
            } else {
                Some(base_string)
            }
        };
        unit_data.equipped_skill_ids = {
            let skill_array = VariantArray::from_variant(&combined_data.at("skill_slots"));
            let mut skill_vec = Vec::with_capacity(skill_array.len());

            for entry in skill_array.iter_shared() {
                skill_vec.push(SkillId::from_variant(&entry));
            }

            skill_vec
        };

        let kit_data = db_link
            .kits
            .get(&unit_data.active_kit_id)
            .expect("Failed to get kit data in unit initializer!");

        unit_data.max_physical_slots = kit_data.weapon_slots;
        unit_data.max_magical_slots = kit_data.magic_slots;
        unit_data.max_support_slots = kit_data.support_slots;
        unit_data.max_item_slots = kit_data.item_slots;

        unit_data.equipped_slot_idx = i8::from_variant(&combined_data.at("equipped_slot_idx"));

        let slot_state_array = VariantArray::from_variant(&combined_data.at("inventory_slots"));
        for (i, state_entry) in slot_state_array.iter_shared().enumerate() {
            unit_data.inventory_slots[i] = InventorySlot::from_variant(&state_entry);
        }

        if unit_data.try_recompute_ranges(db_link) {
            Some(Gd::from_object(unit_data))
        } else {
            None
        }
    }
}
