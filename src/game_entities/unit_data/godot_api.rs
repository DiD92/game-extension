use super::*;

use crate::{
    game_entities::index_store::IndexStore, traits::ToVariantArray, traits::ToVariantOption,
};

#[godot_api]

impl UnitData {
    /// Tries to initialize a new UnitData instance by retrieveing the
    /// definition of 'unit_id' from the game database.
    ///
    /// The method also updates the 'idx_provider' unit and item indexes
    /// on success.
    ///
    /// Current valid 'data_overrides' are the following:
    /// * "current_htp"
    /// * "personality"
    /// * "defend_cell"
    ///
    /// If the initialization fails, the method returns **null**.
    #[func]
    fn from_db(
        unit_id: UnitId,
        db: Gd<DbConnector>,
        mut idx_provider: Gd<IndexStore>,
        data_overrides: Dictionary,
    ) -> Option<Gd<Self>> {
        if let Some(unit_entry) = db.bind().units.get(&unit_id) {
            let mut idx_provider_bind = idx_provider.bind_mut();
            let (next_unit_idx, next_item_idx) = idx_provider_bind.snapshot();

            let unit_idx = idx_provider_bind.next_unit_idx();
            let result = Self::init_from_database(
                unit_id,
                unit_idx,
                unit_entry,
                data_overrides,
                db.bind(),
                &mut idx_provider_bind,
            );

            if result.is_none() {
                idx_provider_bind.rollback_to(next_unit_idx, next_item_idx);
            }

            result
        } else {
            godot_error!("Tried to initialize unit with invalid id [{}]!", unit_id);
            None
        }
    }

    /// TODO: This method has to be replaced with full save state loading
    /// for enemy units, the IndexStore should not be needed when loading for save
    /// that means enemy units need to be stored ENTIRELY on the save data too.
    #[func]
    fn from_db_with_idx(
        unit_id: UnitId,
        unit_idx: UnitIdx,
        db: Gd<DbConnector>,
        mut idx_provider: Gd<IndexStore>,
        data_overrides: Dictionary,
    ) -> Option<Gd<Self>> {
        if let Some(unit_entry) = db.bind().units.get(&unit_id) {
            let mut idx_provider_bind = idx_provider.bind_mut();
            let (next_unit_idx, next_item_idx) = idx_provider_bind.snapshot();

            let result = Self::init_from_database(
                unit_id,
                unit_idx,
                unit_entry,
                data_overrides,
                db.bind(),
                &mut idx_provider_bind,
            );

            if result.is_none() {
                idx_provider_bind.rollback_to(next_unit_idx, next_item_idx);
            }

            result
        } else {
            godot_error!("Tried to initialize unit with invalid id [{}]!", unit_id);
            None
        }
    }

    /// Tries to initialize a new UnitData instance by parsing the data
    /// in `save_data`.
    ///
    /// For details about the format take a look at `UnitSaveData`.
    ///
    /// If the initialization fails, the method returns **null**.
    #[func]
    fn from_save(
        save_data: Dictionary,
        db: Gd<DbConnector>,
        data_overrides: Dictionary,
    ) -> Option<Gd<Self>> {
        if !save_data.is_empty() {
            Self::init_from_save(save_data, db.bind(), data_overrides)
        } else {
            godot_error!("Tried to initialize unit with empty save data!");
            None
        }
    }

    #[func]
    fn get_current_max_htp(&self) -> u8 {
        self.base_htp.saturating_add_signed(self.mod_htp)
    }

    #[func]
    fn get_current_str(&self) -> u8 {
        self.base_str.saturating_add_signed(self.mod_str)
    }

    #[func]
    fn get_current_mag(&self) -> u8 {
        self.base_mag.saturating_add_signed(self.mod_mag)
    }

    #[func]
    fn get_current_def(&self) -> u8 {
        self.base_def.saturating_add_signed(self.mod_def)
    }

    #[func]
    fn get_current_spt(&self) -> u8 {
        self.base_spt.saturating_add_signed(self.mod_spt)
    }

    #[func]
    fn get_current_agi(&self) -> u8 {
        self.base_agi.saturating_add_signed(self.mod_agi)
    }

    #[func]
    fn get_current_dex(&self) -> u8 {
        self.base_dex.saturating_add_signed(self.mod_dex)
    }

    #[func]
    fn get_current_mov(&self) -> u8 {
        self.base_mov.saturating_add_signed(self.mod_mov)
    }

    #[func]
    fn get_current_base_rating(&self) -> u8 {
        self.base_htp
            .saturating_add(self.base_str)
            .saturating_add(self.base_mag)
            .saturating_add(self.base_def)
            .saturating_add(self.base_spt)
            .saturating_add(self.base_agi)
            .saturating_add(self.base_dex)
            .saturating_add(self.base_mov)
    }

    #[func]
    fn get_current_mod_rating(&self) -> i8 {
        self.mod_htp
            .saturating_add(self.mod_str)
            .saturating_add(self.mod_mag)
            .saturating_add(self.mod_def)
            .saturating_add(self.mod_spt)
            .saturating_add(self.mod_agi)
            .saturating_add(self.mod_dex)
            .saturating_add(self.mod_mov)
    }

    #[func]
    fn get_current_effective_rating(&self) -> u8 {
        self.get_current_max_htp()
            .saturating_add(self.get_current_spt())
            .saturating_add(self.get_current_mag())
            .saturating_add(self.get_current_def())
            .saturating_add(self.get_current_spt())
            .saturating_add(self.get_current_agi())
            .saturating_add(self.get_current_dex())
            .saturating_add(self.get_current_mov())
    }

    /// Formula : `(current_dex * 5) + (current_ag * 2)`
    #[func]
    fn get_base_hit(&self) -> u8 {
        self.get_current_dex()
            .saturating_mul(5)
            .saturating_add(self.get_current_agi().saturating_mul(2))
    }

    /// Formula : `(current_agi * 5) + current_dex`
    #[func]
    fn get_base_avoid(&self) -> u8 {
        self.get_current_agi()
            .saturating_mul(5)
            .saturating_add(self.get_current_dex())
    }

    /// Formula : `(current_dex * 2) + current_agi`
    #[func]
    fn get_base_crit(&self) -> u8 {
        self.get_current_dex()
            .saturating_mul(2)
            .saturating_add(self.get_current_agi())
    }

    /// Formula : `current_dex + (current_agi * 2)`
    #[func]
    fn get_base_dodge(&self) -> u8 {
        self.get_current_dex()
            .saturating_add(self.get_current_agi().saturating_mul(2))
    }

    #[func]
    fn can_attack(&self) -> bool {
        self.attack_range > Vector2i::ZERO
    }

    #[func]
    fn can_support(&self) -> bool {
        self.support_range > Vector2i::ZERO
    }

    /// Returns the state of the given `slot_idx` int the format:
    /// `[<slot_id>, <slot_idx>, <slot_uses>]`
    #[func]
    fn get_slot_state(&self, slot_idx: i32) -> VariantArray {
        self.inventory_slots
            .get(slot_idx as usize)
            .and_then(InventorySlot::get_entry)
            .map(SlotEntry::to_godot)
            .unwrap_or_default()
    }

    #[func]
    fn get_slot_id(&self, slot_idx: i32) -> InventoryId {
        if let Some(entry) = self
            .inventory_slots
            .get(slot_idx as usize)
            .and_then(InventorySlot::get_entry)
        {
            entry.id.clone()
        } else {
            InventoryId::default()
        }
    }

    /// Heals 'heal_amt' up to the unit's current maximum hit points.
    /// Returns **true** if the unit is at maximum htp.
    #[func]
    fn receive_heal(&mut self, heal_amt: u8) -> bool {
        let max_htp = self.get_current_max_htp();
        self.current_htp = std::cmp::min(self.current_htp.saturating_add(heal_amt), max_htp);

        self.current_htp == max_htp
    }

    /// Decreases 'dmg_amt' to the unit's current hit points.
    /// Returns **true** if the unit is at 0 htp.
    #[func]
    fn receive_damage(&mut self, dmg_amt: u8) -> bool {
        self.current_htp = std::cmp::max(self.current_htp.saturating_sub(dmg_amt), 0);

        self.current_htp == 0
    }

    /// Tries setting the entry in `slot_idx` as the equipped slot.
    /// Returns **true** if succesfull
    #[func]
    fn try_equip(&mut self, slot_idx: i32) -> bool {
        if let Some(slot) = self.inventory_slots.get(slot_idx as usize) {
            if slot.contains_equipment() && !slot.is_empty() {
                self.equipped_slot_idx = slot_idx as i8;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Tries to exchange the slot at `from_slot_idx` with `to`'s slot at
    /// `to_slot_idx`.
    /// Returns **true** if the exchange succeded.
    #[func]
    fn try_trade_slot_with(
        &mut self,
        from_slot_idx: i32,
        mut to: Gd<UnitData>,
        to_slot_idx: i32,
    ) -> bool {
        let mut to_ref = to.bind_mut();

        match (
            self.inventory_slots.get_mut(from_slot_idx as usize),
            to_ref.inventory_slots.get_mut(to_slot_idx as usize),
        ) {
            (Some(from_slot), Some(to_slot)) => {
                if from_slot.slot_type != to_slot.slot_type {
                    godot_warn!(
                        "Attempted trade with slots from different types! [{:?}] <-> [{:?}]",
                        from_slot.slot_type,
                        to_slot.slot_type
                    );
                    false
                } else if !from_slot.is_empty() || !to_slot.is_empty() {
                    core::mem::swap(from_slot, to_slot);

                    self.recompute_equipped_slot();
                    to_ref.recompute_equipped_slot();

                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    // Clears the unit's inventory slots and returns the item indexes of the cleared slots.
    #[func]
    fn clear_inventory(&mut self) -> Array<InventoryIdx> {
        self.inventory_slots
            .iter_mut()
            .flat_map(InventorySlot::clear_entry)
            .collect::<Array<InventoryIdx>>()
    }

    // Clears the entry in the slot at `slot_idx` and returns the item index if
    // the slot was occupied. Otherwise returns -1.
    #[func]
    fn clear_slot(&mut self, slot_idx: i32) -> i32 {
        self.inventory_slots
            .get_mut(slot_idx as usize)
            .and_then(InventorySlot::clear_entry)
            .map(|item_idx| item_idx as i32)
            .unwrap_or(-1)
    }

    /// Returns **true** if the unit can participate in interactions
    /// no matter what type.
    #[func]
    fn can_interact(&self) -> bool {
        self.inventory_slots
            .iter()
            .any(InventorySlot::contains_equipment)
    }

    /// Gets the database identifier of the unit's equipped personal skill.
    /// Returns an empty string if the unit doesn't have a personal skill equipped.
    #[func]
    fn get_equipped_personal_skill(&self) -> SkillId {
        self.personal_skill_id.clone().unwrap_or_default()
    }

    #[inline]
    fn iter_slots<F>(&self, filter_fn: F) -> Array<Dictionary>
    where
        F: Fn(&InventorySlot) -> bool,
    {
        self.inventory_slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| filter_fn(slot))
            .map(|(slot_idx, slot)| {
                let mut slot_dict = slot.to_godot();
                slot_dict.set("slot_idx", slot_idx as u8);
                slot_dict
            })
            .collect()
    }

    #[func]
    /// Returns array with all unit inventory slots
    fn iter_inventory_slots(&self) -> Array<Dictionary> {
        self.iter_slots(|_| true)
    }

    #[func]
    /// Returns array with all the unit's occupied inventory slots
    fn iter_inventory(&self) -> Array<Dictionary> {
        self.iter_slots(|slot| !slot.is_empty())
    }

    /// Returns array with all the unit's equipment slots,
    /// these are physical, magical and support. Empty or occupied.
    #[func]
    fn iter_equipment(&self) -> Array<Dictionary> {
        self.iter_slots(InventorySlot::contains_equipment)
    }

    /// Returns an array with all the unit's weapon slots.
    ///
    /// Weapon slots include both physical and magical weapons, whether they are empty or occupied.
    ///
    /// # Returns
    ///
    /// An array of dictionaries with the following keys:
    /// * `slot_idx`: The index of the slot in the unit's inventory.
    /// * `type`: The type of the slot, either `physical` or `magical`.
    /// * `entry`: The entry in the slot, if any.
    ///     * 0: The slot id.
    ///     * 1: The slot index.
    ///     * 2: The slot uses.
    #[func]
    fn iter_weapons(&self) -> Array<Dictionary> {
        self.iter_slots(InventorySlot::contains_weapon)
    }

    /// Returns array with all the unit's support slots. Empty or occupied.
    #[func]
    fn iter_support(&self) -> Array<Dictionary> {
        self.iter_slots(InventorySlot::contains_support)
    }

    /// Returns array with all the unit's item slots. Empty or occupied.
    #[func]
    fn iter_items(&self) -> Array<Dictionary> {
        self.iter_slots(InventorySlot::contains_item)
    }

    /// Returns array with all the unit's occupied skills slots.
    #[func]
    fn iter_skills(&self) -> Array<Dictionary> {
        self.personal_skill_id
            .iter()
            .cloned()
            .map(|skill_id| {
                dict! {
                    "skill_id": skill_id,
                    "slot_idx": -1,
                }
            })
            .chain(self.equipped_skill_ids.iter().cloned().enumerate().map(
                |(slot_idx, skill_id)| {
                    dict! {
                        "skill_id": skill_id,
                        "slot_idx": slot_idx as u8
                    }
                },
            ))
            .collect()
    }

    /// Returns a `Dictionary` representation of the `UnitData`'s instance.
    /// Used for storing/loading a unit's save state.
    #[func]
    fn serialize(&self) -> Dictionary {
        let inventory_slots = self
            .inventory_slots
            .iter()
            .map(InventorySlot::to_variant)
            .collect::<VariantArray>();

        dict! {
            "unit_id": self.unit_id.clone(),
            "unit_idx": self.unit_idx,
            "level": self.level,
            "exp": self.experience,
            "base_htp": self.base_htp,
            "base_str": self.base_str,
            "base_mag": self.base_mag,
            "base_def": self.base_def,
            "base_spt": self.base_spt,
            "base_agi": self.base_agi,
            "base_dex": self.base_dex,
            "base_mov": self.base_mov,
            "active_role": self.active_role_id.clone(),
            "active_kit": self.active_kit_id.clone(),
            "personal_skill": self.personal_skill_id.maybe_to_variant(),
            "skill_slots": self.equipped_skill_ids.to_variant_array(),
            "equipped_slot_idx": self.equipped_slot_idx,
            "inventory_slots": inventory_slots,
        }
    }

    #[func]
    fn duplicate(&self) -> Gd<Self> {
        Gd::from_object(self.clone())
    }
}
