use godot::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) type DialogueKey = StringName;
pub(crate) type DialogueSection = StringName;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) struct DialogueConfig {
    key: DialogueKey,
    section: DialogueSection,
    #[serde(default)]
    background_id: StringName,
    #[serde(default)]
    music_id: StringName,
}

impl GodotConvert for DialogueConfig {
    type Via = Dictionary;
}

impl ToGodot for DialogueConfig {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "key": self.key.clone(),
            "section": self.section.clone(),
            "background_id": self.background_id.clone(),
            "music_id": self.music_id.clone(),
        }
    }
}

#[cfg(feature = "verify_database")]
pub(super) mod verify {
    use godot::{
        classes::ResourceLoader,
        global::{godot_error, godot_warn},
    };

    use crate::database::{DbConnector, chapter::ChapterKey};

    use super::DialogueConfig;

    const BASE_DIALOGUE_PATH: &str = "res://dialogue/";
    const BASE_DIALOGUE_BACKGROUND_PATH: &str = "res://assets/art/dialogue_backgrounds/";
    const BASE_DIALOGUE_MUSIC_PATH: &str = "res://assets/sound/music/";

    impl DialogueConfig {
        pub(crate) fn validate(
            &self,
            segment_idx: usize,
            chapter_key: &ChapterKey,
            _db: &DbConnector,
        ) -> bool {
            let dialogue_file_path = format!(
                "{}/{}/{}.dialogue",
                BASE_DIALOGUE_PATH, chapter_key, self.key
            );
            if !ResourceLoader::singleton().exists(&dialogue_file_path) {
                godot_error!(
                    "[{}][{}] Could not find dialogue file [{}]",
                    chapter_key,
                    segment_idx,
                    dialogue_file_path,
                );
                return false;
            }

            if !self.section.is_empty() {
                // TODO: Find a way to check for the section in the dialogue_file_path
            }

            if !self.background_id.is_empty() {
                let dialogue_background_path = format!(
                    "{}/{}.png",
                    BASE_DIALOGUE_BACKGROUND_PATH, self.background_id
                );
                if !ResourceLoader::singleton().exists(&dialogue_background_path) {
                    godot_error!(
                        "[{}][{}] Could not find dialogue background [{}]",
                        chapter_key,
                        segment_idx,
                        dialogue_background_path
                    );
                    return false;
                }
            }

            if !self.music_id.is_empty() {
                let dialogue_music_path =
                    format!("{}/{}.wav", BASE_DIALOGUE_MUSIC_PATH, self.music_id);
                if !ResourceLoader::singleton().exists(&dialogue_music_path) {
                    godot_error!(
                        "[{}][{}] Could not find music [{}]",
                        chapter_key,
                        segment_idx,
                        dialogue_music_path
                    );
                    return false;
                }
            } else {
                godot_warn!(
                    "[{}][{}] Empty 'music_id' found for dialogue segment, are you sure its fine?",
                    chapter_key,
                    segment_idx,
                );
            }

            true
        }
    }
}
