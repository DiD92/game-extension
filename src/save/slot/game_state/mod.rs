use crate::{
    database::{chapter::ChapterKey, unit::UnitId},
    traits::GetVariantOr,
};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

mod battle;
mod camp;
mod dialogue;
mod flags;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "params")]
enum ChapterSegment {
    Dialogue(dialogue::DialogueState),
    Camp(camp::CampState),
    Battle(Box<battle::BattleState>),
}

impl ChapterSegment {
    pub(super) fn get_summary_details(&self) -> Dictionary {
        let mut dict = Dictionary::new();

        match self {
            ChapterSegment::Dialogue(_) => {
                dict.set("title", "Conversing");
            }
            ChapterSegment::Camp(_) => {
                dict.set("title", "In Camp");
            }
            ChapterSegment::Battle(state) => {
                dict.set("title", "In Battle");
                dict.set("current_turn", state.current_turn);
            }
        }

        dict
    }
}

impl GodotConvert for ChapterSegment {
    type Via = Dictionary;
}

impl ToGodot for ChapterSegment {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut state_dict = Dictionary::new();

        match self {
            ChapterSegment::Dialogue(state) => {
                state_dict.set("type", 0);
                state_dict.set("params", state.clone());
            }
            ChapterSegment::Camp(state) => {
                state_dict.set("type", 1);
                state_dict.set("params", state.clone());
            }
            ChapterSegment::Battle(state) => {
                state_dict.set("type", 2);
                state_dict.set("params", state.to_variant());
            }
        }

        state_dict
    }
}

impl FromGodot for ChapterSegment {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        match u8::from_variant(&via.at("type")) {
            0 => Self::Dialogue(dialogue::DialogueState::from_variant(&via.at("params"))),
            1 => Self::Camp(camp::CampState::from_variant(&via.at("params"))),
            2 => Self::Battle(Box::new(battle::BattleState::from_variant(
                &via.at("params"),
            ))),
            _ => panic!("Invalid state key received for saving ChapterState!"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct ChapterSaveState {
    pub(super) key: ChapterKey,
    next_key: ChapterKey,
    current_segment_idx: u8,
    current_segment: ChapterSegment,
}

impl ChapterSaveState {
    pub(super) fn get_summary(&self) -> Dictionary {
        self.current_segment.get_summary_details()
    }

    pub(super) fn update_data(&mut self, update_data: Dictionary) {
        if let Some(data) = update_data.get("key") {
            self.key = ChapterKey::from_variant(&data);
        }

        if let Some(data) = update_data.get("next_key") {
            self.next_key = ChapterKey::from_variant(&data);
        }

        if let Some(data) = update_data.get("current_segment_idx") {
            self.current_segment_idx = u8::from_variant(&data);
        }

        if let Some(data) = update_data.get("current_segment") {
            self.current_segment = ChapterSegment::from_variant(&data);
        }
    }
}

impl GodotConvert for ChapterSaveState {
    type Via = Dictionary;
}

impl ToGodot for ChapterSaveState {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut flag_dict = Dictionary::new();

        flag_dict.set("key", self.key.to_variant());
        flag_dict.set("next_key", self.next_key.to_variant());

        flag_dict.set("current_segment_idx", self.current_segment_idx.to_variant());
        flag_dict.set("current_segment", self.current_segment.to_variant());

        flag_dict
    }
}

impl FromGodot for ChapterSaveState {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            key: ChapterKey::from_variant(&via.at("key")),
            next_key: ChapterKey::from_variant(&via.at("next_key")),
            current_segment_idx: u8::from_variant(&via.at("current_segment_idx")),
            current_segment: ChapterSegment::from_variant(&via.at("current_segment")),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct GameState {
    story_flags: flags::StoryFlags,
    character_flags: flags::CharacterFlags,
    pub(super) chapter_state: ChapterSaveState,
}

impl GameState {
    pub(super) fn update_data(&mut self, update_data: Dictionary) {
        if let Some(data) = update_data.get("story_flags") {
            self.story_flags = flags::StoryFlags::from_variant(&data);
        }

        if let Some(data) = update_data.get("character_flags") {
            self.character_flags = Dictionary::from_variant(&data)
                .iter_shared()
                .map(|(character_id, flag)| {
                    (
                        UnitId::from_variant(&character_id),
                        bool::from_variant(&flag),
                    )
                })
                .collect();
        }

        if let Some(data) = update_data.get("chapter_state") {
            self.chapter_state
                .update_data(Dictionary::from_variant(&data));
        }
    }
}

impl GodotConvert for GameState {
    type Via = Dictionary;
}

impl ToGodot for GameState {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut state_dict = Dictionary::new();

        state_dict.set("story_flags", self.story_flags.to_variant());
        state_dict.set(
            "character_flags",
            self.character_flags
                .iter()
                .map(|(k, f)| (k.to_variant(), f.to_variant()))
                .collect::<Dictionary>(),
        );
        state_dict.set("chapter_state", self.chapter_state.to_godot());

        state_dict
    }
}

impl FromGodot for GameState {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            story_flags: if let Some(data) = via.get("story_flags") {
                flags::StoryFlags::from_variant(&data)
            } else {
                flags::StoryFlags::initial_state()
            },
            character_flags: Dictionary::from_variant(
                &via.get_or("character_flags", Dictionary::default()),
            )
            .iter_shared()
            .map(|(character_id, flag)| {
                (
                    UnitId::from_variant(&character_id),
                    bool::from_variant(&flag),
                )
            })
            .collect(),
            chapter_state: ChapterSaveState::from_variant(&via.at("chapter_state")),
        }
    }
}
