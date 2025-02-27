use super::GetVariantOr;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct CampState {
    background_id: StringName,
    music_id: StringName,
}

impl GodotConvert for CampState {
    type Via = Dictionary;
}

impl ToGodot for CampState {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut dict = Dictionary::new();

        dict.set("background_id", self.background_id.clone());
        dict.set("music_id", self.music_id.clone());

        dict
    }
}

impl FromGodot for CampState {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            background_id: StringName::from_variant(&via.get_or("background_id", "")),
            music_id: StringName::from_variant(&via.get_or("music_id", "")),
        }
    }
}
