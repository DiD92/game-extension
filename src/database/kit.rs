use super::{DbId, DbTable, IdColumn, NameDescColumns};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) type KitId = DbId;

#[derive(Serialize, Deserialize)]
pub(crate) struct KitEntry {
    #[serde(flatten)]
    pub(crate) _i: IdColumn,
    #[serde(flatten)]
    pub(crate) _n: NameDescColumns,
    #[serde(default)]
    pub(crate) htp_mod: i8,
    #[serde(default)]
    pub(crate) str_mod: i8,
    #[serde(default)]
    pub(crate) mag_mod: i8,
    #[serde(default)]
    pub(crate) def_mod: i8,
    #[serde(default)]
    pub(crate) spt_mod: i8,
    #[serde(default)]
    pub(crate) agi_mod: i8,
    #[serde(default)]
    pub(crate) mov_mod: i8,
    #[serde(default)]
    pub(crate) weapon_slots: u8,
    #[serde(default)]
    pub(crate) magic_slots: u8,
    #[serde(default)]
    pub(crate) support_slots: u8,
    #[serde(default)]
    pub(crate) item_slots: u8,
    #[serde(default)]
    pub(crate) skill_slots: u8,
}

impl DbTable for KitEntry {
    fn get_id(&self) -> DbId {
        self._i._id.clone()
    }
}

impl GodotConvert for KitEntry {
    type Via = Dictionary;
}

impl ToGodot for KitEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "id": self._i._id.clone(),
            "name": self._n.name.clone(),
            "description": self._n.description.clone(),
            "htp_mod": self.htp_mod,
            "str_mod": self.str_mod,
            "mag_mod": self.mag_mod,
            "def_mod": self.def_mod,
            "spt_mod": self.spt_mod,
            "agi_mod": self.agi_mod,
            "mov_mod": self.mov_mod,
            "weapon_slots": self.weapon_slots,
            "magic_slots": self.magic_slots,
            "support_slots": self.support_slots,
            "item_slots": self.item_slots,
            "skill_slots": self.skill_slots,
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::KitEntry;
    use crate::database::{DbConnector, validation::VerifyTable};

    use godot::global::godot_error;

    const MAX_SKILL_SLOTS: u8 = 2;
    const TOTAL_SLOT_COUNT: u8 = 6;

    impl VerifyTable for KitEntry {
        fn validate(&self, _db: &DbConnector) -> bool {
            if self.htp_mod < -50 || self.htp_mod > 50 {
                godot_error!(
                    "[{}] Invalid 'htp_mod' value, must be between -50 and 50",
                    self._i._id
                );
                return false;
            }

            if self.str_mod < -50 || self.str_mod > 50 {
                godot_error!(
                    "[{}] Invalid 'str_mod' value, must be between -50 and 50",
                    self._i._id
                );
                return false;
            }

            if self.mag_mod < -50 || self.mag_mod > 50 {
                godot_error!(
                    "[{}] Invalid 'mag_mod' value, must be between -50 and 50",
                    self._i._id
                );
                return false;
            }

            if self.def_mod < -50 || self.def_mod > 50 {
                godot_error!(
                    "[{}] Invalid 'def_mod' value, must be between -50 and 50",
                    self._i._id
                );
                return false;
            }

            if self.spt_mod < -50 || self.spt_mod > 50 {
                godot_error!(
                    "[{}] Invalid 'spt_mod' value, must be between -50 and 50",
                    self._i._id
                );
                return false;
            }

            if self.agi_mod < -50 || self.agi_mod > 50 {
                godot_error!(
                    "[{}] Invalid 'agi_mod' value, must be between -50 and 50",
                    self._i._id
                );
                return false;
            }

            if self.mov_mod < -50 || self.mov_mod > 50 {
                godot_error!(
                    "[{}] Invalid 'mov_mod' value, must be between -50 and 50",
                    self._i._id
                );
                return false;
            }
            if self._i._id.is_empty() || self._n.name.is_empty() || self._n.description.is_empty() {
                godot_error!("[{}] Invalid kit row in database!", self._i._id);
                return false;
            }

            if self.skill_slots > MAX_SKILL_SLOTS {
                godot_error!(
                    "[{}] Invalid 'skill_slots' count, cannot be greater than 2",
                    self._i._id
                );
                return false;
            }

            let slot_count =
                self.weapon_slots + self.magic_slots + self.support_slots + self.item_slots;
            if slot_count != TOTAL_SLOT_COUNT {
                godot_error!(
                    "[{}] Invalid kit slot count total, should be exactly {}",
                    self._i._id,
                    TOTAL_SLOT_COUNT
                );
                return false;
            }

            true
        }
    }
}
