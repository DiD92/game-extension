use super::{
    army_states::ArmyStates,
    unit_data::{UnitData, UnitIdx},
};
use crate::{
    database::{army::ArmyId, personality::PersonalityId},
    traits::FromGstringVariant,
};

use godot::prelude::*;
use std::collections::{BTreeSet, HashMap};

type UnitSet = BTreeSet<UnitIdx>;

#[derive(GodotClass, Default, Clone)]
#[class(no_init, base=RefCounted)]
struct UnitStates {
    pub(crate) unit_idx_to_army_id: HashMap<UnitIdx, ArmyId>,
    pub(crate) army_units: HashMap<ArmyId, UnitSet>,
    pub(crate) defeated_units: HashMap<ArmyId, UnitSet>,
    pub(crate) data_store: HashMap<UnitIdx, Gd<UnitData>>,
    pub(crate) unit_idx_to_cell: HashMap<UnitIdx, Vector2i>,
    pub(crate) grid_cell_to_idx: HashMap<Vector2i, UnitIdx>,
    pub(crate) unit_personalities: HashMap<UnitIdx, PersonalityId>,
    pub(crate) unit_defend_cells: HashMap<UnitIdx, Vector2i>,
}

impl UnitStates {
    /// This method panics if the params used to call `try_from_state`
    /// are somewat invalid
    fn validate_state_params(
        army_units: &Dictionary,
        defeated_units: &Dictionary,
        data_store: &Dictionary,
        unit_idx_to_cell: &Dictionary,
        unit_personalities: &Dictionary,
        unit_defend_cells: &Dictionary,
    ) -> bool {
        use std::collections::HashSet;

        let mut is_valid = true;

        if army_units.is_empty() {
            godot_error!("UnitStates has empty 'army_units' parameter!");
            is_valid = false;
        }

        if defeated_units.is_empty() {
            godot_error!("UnitStates has empty 'defeated_units' parameter!");
            is_valid = false;
        }

        if data_store.is_empty() {
            godot_error!("UnitStates has empty 'data_store' parameter!");
            is_valid = false;
        }

        if unit_idx_to_cell.is_empty() {
            godot_error!("UnitStates has empty 'unit_idx_to_cell' parameter!");
            is_valid = false;
        }

        if army_units.keys_array() != defeated_units.keys_array() {
            godot_error!("UnitStates 'army_units' keys and 'defated_units' keys don't match!");
            is_valid = false;
        }

        if unit_personalities.is_empty() {
            godot_error!("UnitStates has empty 'unit_personalities' parameter!");
            is_valid = false;
        }

        if unit_defend_cells.is_empty() {
            godot_error!("UnitStates has empty 'unit_defend_cells' parameter!");
            is_valid = false;
        }

        let unit_data_idxs = data_store
            .iter_shared()
            .map(|(k, _)| UnitIdx::from_variant(&k))
            .collect::<HashSet<_>>();

        let mut army_unit_idxs = HashSet::new();
        let mut used_unit_idxs = unit_data_idxs.clone();

        for (_, army_units) in army_units.iter_shared() {
            let units_dict = Dictionary::from_variant(&army_units);
            for (unit_idx, _) in units_dict.iter_shared() {
                let idx = UnitIdx::from_variant(&unit_idx);
                army_unit_idxs.insert(idx);
                used_unit_idxs.remove(&idx);
                if !unit_data_idxs.contains(&idx) {
                    godot_error!(
                        "UnitStates 'army_units' unit_idx key not present in 'data_store'!"
                    );
                    is_valid = false;
                }

                if !unit_personalities.contains_key(idx) {
                    godot_error!(
                        "UnitStates 'army_units' unit_idx key not present in 'unit_personalities'!"
                    );
                    is_valid = false;
                }

                if !unit_defend_cells.contains_key(idx) {
                    godot_error!(
                        "UnitStates 'army_units' unit_idx key not present in 'unit_defend_cells'!"
                    );
                    is_valid = false;
                }
            }
        }

        for (_, army_units) in defeated_units.iter_shared() {
            let units_dict = Dictionary::from_variant(&army_units);
            for (unit_idx, _) in units_dict.iter_shared() {
                let idx = UnitIdx::from_variant(&unit_idx);
                used_unit_idxs.remove(&idx);
                if army_unit_idxs.contains(&idx) {
                    godot_error!(
                        "UnitStates 'defeated_units' unit_idx key also present in 'army_units'!"
                    );
                    is_valid = false;
                }

                if !unit_data_idxs.contains(&idx) {
                    godot_error!(
                        "UnitStates 'defeated_units' unit_idx key not present in 'data_store'!"
                    );
                    is_valid = false;
                }
            }
        }

        if !used_unit_idxs.is_empty() {
            godot_error!(
                "UnitStates 'data_store' one or more unit_idx keys not present in 'army_units' nor 'defeated_units!"
            );
            is_valid = false;
        }

        let mut cell_set = HashSet::<Vector2i>::new();
        let mut used_unit_idxs = unit_data_idxs.clone();
        for (unit_idx, unit_cell) in unit_idx_to_cell.iter_shared() {
            let idx = UnitIdx::from_variant(&unit_idx);
            used_unit_idxs.remove(&idx);

            if !unit_data_idxs.contains(&idx) {
                godot_error!(
                    "UnitStates 'unit_idx_to_cell' unit_idx key not present in 'data_store'!"
                );
                is_valid = false;
            }

            let cell = Vector2i::from_variant(&unit_cell);

            if !cell_set.contains(&cell) {
                cell_set.insert(cell);
            } else {
                godot_error!("UnitStates 'unit_idx_to_cell' has duplicated cells!");
                is_valid = false;
            }
        }

        if !used_unit_idxs.is_empty() {
            godot_error!(
                "UnitStates 'data_store' one or more unit_idx keys not present in 'unit_idx_to_cell'!"
            );
            is_valid = false;
        }

        is_valid
    }
}

#[godot_api]
impl UnitStates {
    /// Tries to create a new UnitStates instance from the state parameters.
    ///
    /// The expected structure of the parameters is the following:
    /// * **army_units**: `{<army_id>: {<unit_idx>: true}}`
    /// * **defeated_units**: `{<army_id>: {<unit_idx>: true}}`
    /// * **data_store**: `{<unit_idx>: <unit_data: UnitData>}`
    /// * **unit_idx_to_cell**: `{<unit_idx>: <unit_cell: Vector2i>}`
    /// * **unit_personalities**: `{<unit_idx>: <personality_id: PersonalityId>}`
    ///
    /// Will return **null** if the validation of the parameters **didn't succeed!**
    #[func]
    fn try_from_state(
        army_units: Dictionary,
        defeated_units: Dictionary,
        data_store: Dictionary,
        unit_idx_to_cell: Dictionary,
        unit_personalities: Dictionary,
        unit_defend_cells: Dictionary,
    ) -> Option<Gd<Self>> {
        if !Self::validate_state_params(
            &army_units,
            &defeated_units,
            &data_store,
            &unit_idx_to_cell,
            &unit_personalities,
            &unit_defend_cells,
        ) {
            return None;
        }

        let mut states = Self::default();

        for (army_id, army_units) in army_units.iter_shared() {
            let id = ArmyId::from_gstring_variant_to_string_name(&army_id);
            let units_dict = Dictionary::from_variant(&army_units);

            let mut army_units = UnitSet::new();

            for (unit_idx, _) in units_dict.iter_shared() {
                let idx = UnitIdx::from_variant(&unit_idx);

                states.unit_idx_to_army_id.insert(idx, id.clone());
                army_units.insert(idx);
            }

            states.army_units.insert(id.clone(), army_units);
        }

        for (army_id, army_units) in defeated_units.iter_shared() {
            let id = ArmyId::from_gstring_variant_to_string_name(&army_id);
            let units_dict = Dictionary::from_variant(&army_units);

            let mut defeated_set = UnitSet::new();

            for (unit_idx, _) in units_dict.iter_shared() {
                let idx = UnitIdx::from_variant(&unit_idx);
                defeated_set.insert(idx);
            }

            states.defeated_units.insert(id.clone(), defeated_set);
        }

        for (unit_idx, unit_data) in data_store.iter_shared() {
            let idx = UnitIdx::from_variant(&unit_idx);
            let data = Gd::<UnitData>::from_variant(&unit_data);

            states.data_store.insert(idx, data);
        }

        for (unit_idx, unit_cell) in unit_idx_to_cell.iter_shared() {
            let idx = UnitIdx::from_variant(&unit_idx);
            let cell = Vector2i::from_variant(&unit_cell);

            states.unit_idx_to_cell.insert(idx, cell);
            states.grid_cell_to_idx.insert(cell, idx);
        }

        for (unit_idx, personality_id) in unit_personalities.iter_shared() {
            let idx = UnitIdx::from_variant(&unit_idx);
            let id = PersonalityId::from_variant(&personality_id);

            states.unit_personalities.insert(idx, id);
        }

        for (unit_idx, defend_cell) in unit_defend_cells.iter_shared() {
            let idx = UnitIdx::from_variant(&unit_idx);
            let cell = Vector2i::from_variant(&defend_cell);

            states.unit_defend_cells.insert(idx, cell);
        }

        Some(Gd::from_object(states))
    }

    /// Tries to update 'unit_idx' position to 'new_cell'.
    /// Returns **false** if 'new_cell' is already occupied.
    #[func]
    fn try_update_position_to(&mut self, unit_idx: UnitIdx, new_cell: Vector2i) -> bool {
        if self.grid_cell_to_idx.contains_key(&new_cell) {
            godot_error!("Cell {} already occupied", new_cell);
            false
        } else {
            if let Some(old_cell) = self.unit_idx_to_cell.insert(unit_idx, new_cell) {
                let _ = self.grid_cell_to_idx.remove(&old_cell);
            }

            let _ = self.grid_cell_to_idx.insert(new_cell, unit_idx);

            true
        }
    }

    #[func]
    fn mark_unit_as_defeated(&mut self, army_id: ArmyId, unit_idx: UnitIdx) {
        if let Some(unit_set) = self.army_units.get_mut(&army_id) {
            if unit_set.remove(&unit_idx) {
                if let Some(defeated_set) = self.defeated_units.get_mut(&army_id) {
                    defeated_set.insert(unit_idx);
                }
            }
        }

        if let Some(defeated_at) = self.unit_idx_to_cell.remove(&unit_idx) {
            self.grid_cell_to_idx.remove(&defeated_at);
        }
    }

    #[func]
    fn has_enemy_in_cell(
        &self,
        unit_idx: UnitIdx,
        cell: Vector2i,
        army_states_link: Gd<ArmyStates>,
    ) -> bool {
        match (
            self.unit_idx_to_army_id.get(&unit_idx),
            self.grid_cell_to_idx.get(&cell),
        ) {
            (Some(unit_army_id), Some(other_idx)) => {
                if unit_idx == *other_idx {
                    false
                } else {
                    let army_states = army_states_link.bind();
                    let other_army_id = &self.unit_idx_to_army_id[other_idx];

                    if unit_army_id == other_army_id {
                        false
                    } else if unit_army_id == &army_states.player_army
                        || army_states.allied_armies.contains(unit_army_id)
                    {
                        army_states.enemy_armies.contains(other_army_id)
                    } else if army_states.enemy_armies.contains(unit_army_id) {
                        army_states.allied_armies.contains(other_army_id)
                            || other_army_id == &army_states.player_army
                    } else {
                        false
                    }
                }
            }
            _ => false,
        }
    }

    #[func]
    fn has_ally_in_cell(
        &self,
        unit_idx: UnitIdx,
        cell: Vector2i,
        army_states_link: Gd<ArmyStates>,
    ) -> bool {
        match (
            self.unit_idx_to_army_id.get(&unit_idx),
            self.grid_cell_to_idx.get(&cell),
        ) {
            (Some(unit_army_id), Some(other_idx)) => {
                if unit_idx == *other_idx {
                    false
                } else {
                    let army_states = army_states_link.bind();
                    let other_army_id = &self.unit_idx_to_army_id[other_idx];

                    if unit_army_id == other_army_id {
                        true
                    } else if unit_army_id == &army_states.player_army {
                        army_states.allied_armies.contains(other_army_id)
                    } else if army_states.allied_armies.contains(unit_army_id) {
                        army_states.allied_armies.contains(other_army_id)
                            || other_army_id == &army_states.player_army
                    } else if army_states.enemy_armies.contains(unit_army_id) {
                        army_states.enemy_armies.contains(other_army_id)
                    } else {
                        false
                    }
                }
            }
            _ => false,
        }
    }

    #[func]
    fn has_companion_in_cell(&self, unit_idx: UnitIdx, cell: Vector2i) -> bool {
        match (
            self.unit_idx_to_army_id.get(&unit_idx),
            self.grid_cell_to_idx.get(&cell),
        ) {
            (Some(unit_army_id), Some(other_idx)) => {
                if unit_idx == *other_idx {
                    false
                } else {
                    let other_army_id = &self.unit_idx_to_army_id[other_idx];

                    unit_army_id == other_army_id
                }
            }
            _ => false,
        }
    }

    /// Tries to retrieve the associated 'ArmyId' of the 'unit_idx'.
    /// Returns and empty ArmyId if the 'unit_idx' could not be found.
    #[func]
    fn try_get_army_id_for(&self, unit_idx: UnitIdx) -> ArmyId {
        if let Some(army_id) = self.unit_idx_to_army_id.get(&unit_idx) {
            army_id.clone()
        } else {
            ArmyId::default()
        }
    }

    #[func]
    fn iter_army_ids(&self) -> Array<ArmyId> {
        self.army_units.keys().cloned().collect()
    }

    /// Returns the amount of units present in 'army_units' for
    /// 'army_id.
    #[func]
    fn unit_count_for(&self, army_id: ArmyId) -> u32 {
        self.army_units
            .get(&army_id)
            .map(|units| units.len() as u32)
            .unwrap_or_default()
    }

    #[func]
    fn contains_unit(&self, army_id: ArmyId, unit_idx: UnitIdx) -> bool {
        self.army_units
            .get(&army_id)
            .map(|units| units.contains(&unit_idx))
            .unwrap_or_default()
    }

    /// Tries to get the unit_idx of the previous unit in the army 'army_id'.
    /// Returns **-1** if a previous unit could not be found.
    #[func]
    fn try_get_prev_unit_order_idx_for(&self, army_id: ArmyId, unit_idx: UnitIdx) -> i32 {
        if !self.army_units.contains_key(&army_id) {
            return -1;
        }

        if let Some((order_idx, _)) = self.army_units[&army_id]
            .iter()
            .enumerate()
            .find(|(_, idx)| *idx == &unit_idx)
        {
            let unit_count = self.army_units[&army_id].len() as isize;
            self.army_units[&army_id]
                .iter()
                .nth(((order_idx as isize) - 1).rem_euclid(unit_count) as usize)
                .map(|idx| *idx as i32)
                .unwrap_or(-1)
        } else {
            -1
        }
    }

    /// Tries to get the unit_idx of the next unit in the army 'army_id'.
    /// Returns **-1** if a previous unit could not be found.
    #[func]
    fn try_get_next_unit_order_idx_for(&self, army_id: ArmyId, unit_idx: UnitIdx) -> i32 {
        if !self.army_units.contains_key(&army_id) {
            return -1;
        }

        if let Some((order_idx, _)) = self.army_units[&army_id]
            .iter()
            .enumerate()
            .find(|(_, idx)| *idx == &unit_idx)
        {
            let unit_count = self.army_units[&army_id].len();
            self.army_units[&army_id]
                .iter()
                .nth((order_idx + 1) % unit_count)
                .map(|idx| *idx as i32)
                .unwrap_or(-1)
        } else {
            -1
        }
    }

    #[func]
    fn iter_army_units_for(&self, army_id: ArmyId) -> Array<UnitIdx> {
        self.army_units
            .get(&army_id)
            .map(|units| units.iter().copied().collect::<Array<UnitIdx>>())
            .unwrap_or_default()
    }

    #[func]
    fn iter_defeated_army_ids(&self) -> Array<ArmyId> {
        self.defeated_units.keys().cloned().collect()
    }

    #[func]
    fn iter_defeated_units_for(&self, army_id: ArmyId) -> Array<UnitIdx> {
        self.defeated_units
            .get(&army_id)
            .map(|units| units.iter().copied().collect::<Array<UnitIdx>>())
            .unwrap_or_default()
    }

    /// Tries to get the UnidData associated with 'unit_idx'.
    /// Returns **null** if 'unit_idx' could not be found.
    #[func]
    fn try_get_data_for(&self, unit_idx: UnitIdx) -> Option<Gd<UnitData>> {
        self.data_store.get(&unit_idx).cloned()
    }

    /// Tries to get the Vector2i cell occupied by 'unit_idx'.
    /// Returns Vector2i(-1, -1) if 'unit_idx' could not be found.
    #[func]
    fn try_get_cell_for(&self, unit_idx: UnitIdx) -> Vector2i {
        self.unit_idx_to_cell
            .get(&unit_idx)
            .copied()
            .unwrap_or(Vector2i::new(-1, -1))
    }

    /// Returns an array of all cells occupied by a unit.
    /// The array elements are **not ordered**.
    #[func]
    fn iter_unit_coords(&self) -> Array<Vector2i> {
        self.grid_cell_to_idx.keys().copied().collect()
    }

    #[func]
    fn has_unit_at_cell(&self, cell: Vector2i) -> bool {
        self.grid_cell_to_idx.contains_key(&cell)
    }

    /// Tries to get the unit_idx occupying 'cell'.
    /// Returns -1 if 'cell' is not occupied.
    #[func]
    fn try_get_unit_idx_for(&self, cell: Vector2i) -> i32 {
        self.grid_cell_to_idx
            .get(&cell)
            .copied()
            .map(|unit_idx| unit_idx as i32)
            .unwrap_or(-1)
    }

    /// Tries to get the PersonalityId associated with 'unit_idx'.
    /// Returns an empty PersonalityId if 'unit_idx' could not be found.
    #[func]
    fn try_get_personality_for(&self, unit_idx: UnitIdx) -> PersonalityId {
        self.unit_personalities
            .get(&unit_idx)
            .cloned()
            .unwrap_or_default()
    }

    /// Tries to get the defend cell associated with 'unit_idx'.
    /// Returns Vector2i(-1, -1) if 'unit_idx' could not be found.
    #[func]
    fn try_get_defend_cell_for(&self, unit_idx: UnitIdx) -> Vector2i {
        self.unit_defend_cells
            .get(&unit_idx)
            .copied()
            .unwrap_or(Vector2i::new(-1, -1))
    }
}
