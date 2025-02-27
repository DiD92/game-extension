use super::{DbId, DbTable, IdColumn, NameDescColumns};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

mod entry_type;
mod entry_uses;

pub(crate) use entry_type::*;
pub(crate) use entry_uses::EntryUses;

pub(crate) type InventoryId = DbId;

#[derive(Serialize, Deserialize)]
pub(crate) struct InventoryEntry {
    #[serde(flatten)]
    pub(crate) _i: IdColumn,
    #[serde(flatten)]
    pub(crate) _n: NameDescColumns,
    pub(crate) icon_id: StringName,
    #[serde(flatten)]
    pub(crate) _variant: EntryVariant,
    #[serde(default)]
    pub(crate) uses: EntryUses,
    #[serde(default)]
    pub(crate) value: u16,
    #[serde(default)]
    pub(crate) can_sell: bool,
}

impl DbTable for InventoryEntry {
    fn get_id(&self) -> DbId {
        self._i._id.clone()
    }
}

impl GodotConvert for InventoryEntry {
    type Via = Dictionary;
}

impl ToGodot for InventoryEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut entry_dict = dict! {
            "id": self._i._id.clone(),
            "name": self._n.name.clone(),
            "description": self._n.description.clone(),
            "icon_id": self.icon_id.clone(),
            "uses": self.uses.to_godot(),
            "value": self.value,
            "can_sell": self.can_sell,
        };

        entry_dict.extend_dictionary(&self._variant.to_godot(), true);

        entry_dict
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::InventoryEntry;
    use crate::database::{DbConnector, validation::VerifyTable};

    use godot::{classes::ResourceLoader, global::godot_error};

    const BASE_ICON_PATH: &str = "res://assets/art/inventory_icons/";

    impl VerifyTable for InventoryEntry {
        fn validate(&self, db: &DbConnector) -> bool {
            if self._i._id.is_empty() || self._n.is_empty() {
                godot_error!("[{}] Invalid inventory row in database!", self._i._id);
                return false;
            }

            let icon_path = format!("{}{}.png", BASE_ICON_PATH, self.icon_id);
            if !ResourceLoader::singleton().exists(&icon_path) {
                godot_error!("[{}] Could not find icon [{}]!", self._i._id, self.icon_id);
                // TODO: Add return when properly implemented
                // return false;
            }

            if !self._variant.validate(db) {
                godot_error!("[{}] Entry validation failed!", self._i._id);
                return false;
            }

            true
        }
    }
}
