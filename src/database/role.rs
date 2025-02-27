use super::{DbId, DbTable, IdColumn, NameDescColumns};
use crate::traits::ToVariantArray;

use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub(crate) type RoleId = DbId;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum ValorType {
    #[default]
    None = 0,
    Critical = 1,
    Movement = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
#[serde(untagged)]
pub(crate) enum RoleTags {
    Cavalry = 0,
    Flying = 1,
    Armored = 2,
}

impl GodotConvert for RoleTags {
    type Via = u8;
}

impl ToGodot for RoleTags {
    type ToVia<'v> = u8;

    fn to_godot(&self) -> Self::ToVia<'_> {
        *self as u8
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RoleEntry {
    #[serde(flatten)]
    _i: IdColumn,
    #[serde(flatten)]
    _n: NameDescColumns,
    #[serde(default)]
    htp_mod: i8,
    #[serde(default)]
    str_mod: i8,
    #[serde(default)]
    mag_mod: i8,
    #[serde(default)]
    def_mod: i8,
    #[serde(default)]
    spt_mod: i8,
    #[serde(default)]
    agi_mod: i8,
    #[serde(default)]
    mov_mod: i8,
    #[serde(default)]
    valor_type: ValorType,
    #[serde(default)]
    role_tags: HashSet<RoleTags>,
}

impl DbTable for RoleEntry {
    fn get_id(&self) -> DbId {
        self._i._id.clone()
    }
}

impl GodotConvert for RoleEntry {
    type Via = Dictionary;
}

impl ToGodot for RoleEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "id": self._i._id.to_godot(),
            "name": self._n.name.to_godot(),
            "description": self._n.description.to_godot(),
            "htp_mod": self.htp_mod,
            "str_mod": self.str_mod,
            "mag_mod": self.mag_mod,
            "def_mod": self.def_mod,
            "spt_mod": self.spt_mod,
            "agi_mod": self.agi_mod,
            "mov_mod": self.mov_mod,
            "valor_type": self.valor_type as u8,
            "role_tags": self.role_tags.to_variant_array(),
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::RoleEntry;
    use crate::database::{DbConnector, validation::VerifyTable};

    use godot::global::godot_error;

    impl VerifyTable for RoleEntry {
        fn validate(&self, _db: &DbConnector) -> bool {
            if self.htp_mod < -30 || self.htp_mod > 30 {
                godot_error!("[{}] htp_mod out of range: {}", self._i._id, self.htp_mod);
                return false;
            }

            if self.str_mod < -30 || self.str_mod > 30 {
                godot_error!("[{}] str_mod out of range: {}", self._i._id, self.str_mod);
                return false;
            }

            if self.mag_mod < -30 || self.mag_mod > 30 {
                godot_error!("[{}] mag_mod out of range: {}", self._i._id, self.mag_mod);
                return false;
            }

            if self.def_mod < -30 || self.def_mod > 30 {
                godot_error!("[{}] def_mod out of range: {}", self._i._id, self.def_mod);
                return false;
            }

            if self.spt_mod < -30 || self.spt_mod > 30 {
                godot_error!("[{}] spt_mod out of range: {}", self._i._id, self.spt_mod);
                return false;
            }

            if self.agi_mod < -30 || self.agi_mod > 30 {
                godot_error!("[{}] agi_mod out of range: {}", self._i._id, self.agi_mod);
                return false;
            }

            if self.mov_mod < -30 || self.mov_mod > 30 {
                godot_error!("[{}] mov_mod out of range: {}", self._i._id, self.mov_mod);
                return false;
            }

            if self._i._id.is_empty() || self._n.is_empty() {
                godot_error!("[{}] Invalid role row in database!", self._i._id);
                return false;
            }

            true
        }
    }
}
