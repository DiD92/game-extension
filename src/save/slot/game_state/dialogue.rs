use super::GetVariantOr;
use crate::database::chapter::{DialogueKey, DialogueSection};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct BustStates {
    left_0: Option<StringName>,
    left_1: Option<StringName>,
    right_0: Option<StringName>,
    right_1: Option<StringName>,
}

impl GodotConvert for BustStates {
    type Via = Dictionary;
}

impl ToGodot for BustStates {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "left_0": self.left_0.clone().unwrap_or_default(),
            "left_1": self.left_1.clone().unwrap_or_default(),
            "right_0": self.right_0.clone().unwrap_or_default(),
            "right_1": self.right_1.clone().unwrap_or_default(),
        }
    }
}

impl FromGodot for BustStates {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            left_0: via
                .get("left_0")
                .map(|data| StringName::from_variant(&data)),
            left_1: via
                .get("left_1")
                .map(|data| StringName::from_variant(&data)),
            right_0: via
                .get("right_0")
                .map(|data| StringName::from_variant(&data)),
            right_1: via
                .get("right_1")
                .map(|data| StringName::from_variant(&data)),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct DialogueState {
    key: DialogueKey,
    section: DialogueSection,
    bust_states: Option<BustStates>,
    background_id: StringName,
    music_id: StringName,
}

impl GodotConvert for DialogueState {
    type Via = Dictionary;
}

impl ToGodot for DialogueState {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut dict = Dictionary::new();

        dict.set("key", self.key.clone());
        dict.set("section", self.section.clone());

        dict.set("background_id", self.background_id.clone());
        dict.set("music_id", self.music_id.clone());

        dict
    }
}

impl FromGodot for DialogueState {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            key: DialogueKey::from_variant(&via.at("key")),
            section: DialogueSection::from_variant(&via.get_or("section", "")),
            bust_states: via
                .get("bust_states")
                .map(|data| BustStates::from_variant(&data)),
            background_id: StringName::from_variant(&via.get_or("background_id", "")),
            music_id: StringName::from_variant(&via.get_or("music_id", "")),
        }
    }
}
