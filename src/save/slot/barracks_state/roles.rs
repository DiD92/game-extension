use super::{RoleId, UnitId};

use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(super) type UnitRoles = HashMap<RoleId, RoleEntry>;
pub(super) type UnitsRoles = HashMap<UnitId, UnitRoles>;

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct RoleEntry {
    level: u8,
    exp: u8,
}

impl Default for RoleEntry {
    fn default() -> Self {
        Self { level: 1, exp: 0 }
    }
}

impl GodotConvert for RoleEntry {
    type Via = Dictionary;
}

impl ToGodot for RoleEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut entry_dict = Dictionary::new();

        entry_dict.set("level", self.level);
        entry_dict.set("exp", self.exp);

        entry_dict
    }
}

impl FromGodot for RoleEntry {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            level: u8::from_variant(&via.at("level")),
            exp: u8::from_variant(&via.at("exp")),
        }
    }
}
