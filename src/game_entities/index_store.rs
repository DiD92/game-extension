use super::unit_data::{InventoryIdx, UnitIdx};

use godot::prelude::*;

#[derive(GodotClass, Default)]
#[class(no_init, base=RefCounted)]
pub(crate) struct IndexStore {
    #[var]
    pub(crate) next_unit_idx: UnitIdx,
    #[var]
    pub(crate) next_item_idx: InventoryIdx,
}

impl IndexStore {
    pub(crate) fn snapshot(&self) -> (u32, u32) {
        (self.next_unit_idx, self.next_item_idx)
    }
}

#[godot_api]
impl IndexStore {
    #[func]
    fn with_initial_values(unit_idx: u32, item_idx: u32) -> Gd<Self> {
        Gd::from_object(Self {
            next_unit_idx: unit_idx,
            next_item_idx: item_idx,
        })
    }

    #[func]
    pub(crate) fn rollback_to(&mut self, unit_idx: u32, item_idx: u32) {
        self.next_unit_idx = unit_idx;
        self.next_item_idx = item_idx;
    }

    /// Returns and increases the unit_idx internal counter
    #[func]
    pub(crate) fn next_unit_idx(&mut self) -> UnitIdx {
        let idx = self.next_unit_idx;
        self.next_unit_idx = self.next_unit_idx.wrapping_add(1);
        idx
    }

    /// Returns and increases the item_idx internal counter
    #[func]
    pub(crate) fn next_item_idx(&mut self) -> InventoryIdx {
        let idx = self.next_item_idx;
        self.next_item_idx = self.next_item_idx.wrapping_add(1);
        idx
    }
}
