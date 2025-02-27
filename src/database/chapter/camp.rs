use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) struct CampConfig {
    background_id: StringName,
    music_id: StringName,
}

impl GodotConvert for CampConfig {
    type Via = Dictionary;
}

impl ToGodot for CampConfig {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "background_id": self.background_id.clone(),
            "music_id": self.music_id.clone(),
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::CampConfig;
    use crate::database::{DbConnector, chapter::ChapterKey};

    use godot::{
        classes::ResourceLoader,
        global::{godot_error, godot_warn},
    };

    const BASE_CAMP_BACKGROUND_PATH: &str = "res://assets/art/dialogue_backgrounds/";
    const BASE_CAMP_MUSIC_PATH: &str = "res://assets/sound/music/";

    impl CampConfig {
        pub(crate) fn validate(
            &self,
            segment_idx: usize,
            chapter_key: &ChapterKey,
            _: &DbConnector,
        ) -> bool {
            if !self.background_id.is_empty() {
                let camp_background_path =
                    format!("{}/{}.png", BASE_CAMP_BACKGROUND_PATH, self.background_id);
                if !ResourceLoader::singleton().exists(&camp_background_path) {
                    godot_error!(
                        "[{}][{}] Could not find camp background [{}]",
                        chapter_key,
                        segment_idx,
                        camp_background_path
                    );
                    return false;
                }
            } else {
                godot_warn!("Empty 'background_id' found for camp segment, are you sure its fine?");
            }

            if !self.music_id.is_empty() {
                let camp_music_path = format!("{}/{}.wav", BASE_CAMP_MUSIC_PATH, self.music_id);
                if !ResourceLoader::singleton().exists(&camp_music_path) {
                    godot_error!(
                        "[{}][{}] Could not find music [{}]",
                        chapter_key,
                        segment_idx,
                        camp_music_path
                    );
                    return false;
                }
            } else {
                godot_warn!(
                    "[{}][{}] Empty 'music_id' found for camp segment, are you sure its fine?",
                    chapter_key,
                    segment_idx,
                );
            }

            true
        }
    }
}
