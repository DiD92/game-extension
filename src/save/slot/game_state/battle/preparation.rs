use super::DialogueState;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct PreparationsState {
    pre_dialogue_state: Option<DialogueState>,
    music_id: StringName,
}

impl GodotConvert for PreparationsState {
    type Via = Dictionary;
}

impl ToGodot for PreparationsState {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut dict = Dictionary::new();

        dict.set(
            "pre_dialogue_state",
            self.pre_dialogue_state
                .as_ref()
                .map(|d| d.to_variant())
                .unwrap_or(Dictionary::default().to_variant()),
        );

        dict.set("music_id", self.music_id.to_variant());

        dict
    }
}

impl FromGodot for PreparationsState {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            pre_dialogue_state: via
                .get("pre_dialogue_state")
                .filter(|data| !Dictionary::from_variant(data).is_empty())
                .map(|data| DialogueState::from_variant(&data)),
            music_id: StringName::from_variant(&via.at("music_id")),
        }
    }
}
