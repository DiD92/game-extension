use crate::database::unit::UnitId;

use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(super) type CharacterFlags = HashMap<UnitId, bool>;

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct StoryFlags {
    flag_1: bool,
    flag_2: i32,
}

impl StoryFlags {
    pub(super) fn initial_state() -> Self {
        Self {
            flag_1: false,
            flag_2: 10,
        }
    }
}

impl GodotConvert for StoryFlags {
    type Via = Dictionary;
}

impl ToGodot for StoryFlags {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut flag_dict = Dictionary::new();

        flag_dict.set("flag_1", self.flag_1);
        flag_dict.set("flag_2", self.flag_2);

        flag_dict
    }
}

impl FromGodot for StoryFlags {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            flag_1: bool::from_variant(&via.at("flag_1")),
            flag_2: i32::from_variant(&via.at("flag_2")),
        }
    }
}
