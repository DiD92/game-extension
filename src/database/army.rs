use super::{DbId, DbTable, IdColumn, NameDescColumns};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) type ArmyId = DbId;

#[derive(Serialize, Deserialize)]
pub(crate) struct ArmyEntry {
    #[serde(flatten)]
    _i: IdColumn,
    #[serde(flatten)]
    _n: NameDescColumns,
    banner_id: StringName,
}

impl DbTable for ArmyEntry {
    fn get_id(&self) -> DbId {
        self._i._id.clone()
    }
}

impl GodotConvert for ArmyEntry {
    type Via = Dictionary;
}

impl ToGodot for ArmyEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "id": self._i._id.clone(),
            "name": self._n.name.clone(),
            "description": self._n.description.clone(),
            "banner_id": self.banner_id.clone(),
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::ArmyEntry;
    use crate::database::{DbConnector, validation::VerifyTable};

    use godot::{classes::ResourceLoader, global::godot_error};

    const BASE_BANNER_PATH: &str = "res://assets/art/army_banners/";

    impl VerifyTable for ArmyEntry {
        fn validate(&self, _db: &DbConnector) -> bool {
            if self._i._id.is_empty() || self._n.is_empty() {
                godot_error!("[{}] Invalid army row in database!", self._i._id);
                return false;
            }

            if self._n.name.is_empty() {
                godot_error!("[{}] Army name cannot be empty!", self._i._id);
                return false;
            }

            if self._n.description.is_empty() {
                godot_error!("[{}] Army description cannot be empty!", self._i._id);
                return false;
            }

            let banner_path = format!("{}{}.png", BASE_BANNER_PATH, self.banner_id);
            if !ResourceLoader::singleton().exists(&banner_path) {
                godot_error!(
                    "[{}] Could not find army banner [{}]",
                    self._i._id,
                    self.banner_id
                );
                return false;
            }

            true
        }
    }
}
