use crate::{database::inventory::InventoryId, game_entities::unit_data::InventoryIdx};

use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(super) type InventoryEntries = HashMap<InventoryIdx, InventoryEntry>;

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct InventoryEntry {
    inventory_id: InventoryId,
    uses: i8,
}

impl GodotConvert for InventoryEntry {
    type Via = Dictionary;
}

impl ToGodot for InventoryEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut state_dict = Dictionary::new();

        state_dict.set("inventory_id", self.inventory_id.clone());
        state_dict.set("uses", self.uses);

        state_dict
    }
}

impl FromGodot for InventoryEntry {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            inventory_id: InventoryId::from_variant(&via.at("inventory_id")),
            uses: i8::from_variant(&via.at("uses")),
        }
    }
}
