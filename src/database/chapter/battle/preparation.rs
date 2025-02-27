use super::{DialogueConfig, Vector2u8};
use crate::traits::{ToDefaultVariant, ToVariantArray};

use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) struct PreparationConfig {
    #[serde(default)]
    pre_dialogue: Option<DialogueConfig>,
    music_id: StringName,
    preparation_placements: HashSet<Vector2u8>,
}

impl GodotConvert for PreparationConfig {
    type Via = Dictionary;
}

impl ToGodot for PreparationConfig {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "pre_dialogue": self.pre_dialogue.to_default_variant(),
            "music_id": self.music_id.clone(),
            "preparation_placements": self.preparation_placements.to_variant_array(),
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::PreparationConfig;
    use crate::database::{
        DbConnector,
        chapter::{ChapterKey, battle::BattleConfig},
    };

    use godot::{classes::ResourceLoader, global::godot_error};

    const BASE_BATTLE_PREPARATIONS_MUSIC_PATH: &str = "res://assets/sound/music/";

    impl PreparationConfig {
        pub(crate) fn validate(
            &self,
            _: &BattleConfig,
            segment_idx: usize,
            chapter_key: &ChapterKey,
            db: &DbConnector,
        ) -> bool {
            if let Some(ref dialogue) = self.pre_dialogue {
                if !dialogue.validate(segment_idx, chapter_key, db) {
                    return false;
                }
            }

            let battle_preparations_music_path = format!(
                "{}/{}.wav",
                BASE_BATTLE_PREPARATIONS_MUSIC_PATH, self.music_id
            );
            if !ResourceLoader::singleton().exists(&battle_preparations_music_path) {
                godot_error!(
                    "[{}][{}] Could not find music [{}]",
                    chapter_key,
                    segment_idx,
                    battle_preparations_music_path
                );
                return false;
            }

            if self.preparation_placements.is_empty() {
                godot_error!(
                    "[{}][{}] Preparations placements cannot be empty!",
                    chapter_key,
                    segment_idx
                );
                return false;
            }

            true
        }
    }
}
