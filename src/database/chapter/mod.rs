use crate::traits::ToVariantArray;

use super::{DbId, DbTable, IdColumn};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

mod battle;
mod camp;
mod dialogue;

pub(crate) use battle::Vector2u8;
pub(crate) use dialogue::{DialogueKey, DialogueSection};

pub(crate) type ChapterKey = DbId;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
#[serde(tag = "type", content = "params")]
pub(crate) enum ChapterSegment {
    Dialogue(dialogue::DialogueConfig),
    Camp(camp::CampConfig),
    Battle(Box<battle::BattleConfig>),
}

impl GodotConvert for ChapterSegment {
    type Via = Dictionary;
}

impl ToGodot for ChapterSegment {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let (type_code, entry_params) = match self {
            ChapterSegment::Dialogue(config) => (0_u8, config.to_variant()),
            ChapterSegment::Camp(config) => (1_u8, config.to_variant()),
            ChapterSegment::Battle(config) => (2_u8, config.to_variant()),
        };

        dict! {
            "type": type_code,
            "params": entry_params,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ChapterEntry {
    #[serde(flatten)]
    _i: IdColumn,
    code: GString,
    title: GString,
    background_id: GString,
    segments: Vec<ChapterSegment>,
    next_chapter: ChapterKey,
}

impl DbTable for ChapterEntry {
    fn get_id(&self) -> DbId {
        self._i._id.clone()
    }
}

impl GodotConvert for ChapterEntry {
    type Via = Dictionary;
}

impl ToGodot for ChapterEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "id": self._i._id.clone(),
            "code": self.code.clone(),
            "title": self.title.clone(),
            "background_id": self.background_id.clone(),
            "segments": self.segments.to_variant_array(),
            "next_chapter": self.next_chapter.clone(),
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::{ChapterEntry, ChapterKey, ChapterSegment};
    use crate::database::{DbConnector, validation::VerifyTable};

    use godot::{classes::ResourceLoader, global::godot_error};

    const NEXT_CHAPTER_MAIN_MENU: &str = "main_menu";
    const BASE_BACKGROUND_PATH: &str = "res://assets/art/chapter_backgrounds/";

    impl VerifyTable for ChapterEntry {
        fn validate(&self, db: &DbConnector) -> bool {
            if self._i._id.is_empty() {
                godot_error!("[{}] Invalid chapter row in database!", self._i._id);
                return false;
            }

            if self.code.is_empty() || self.title.is_empty() {
                godot_error!("[{}] Chapter code and title cannot be empty!", self._i._id);
                return false;
            }

            let background_path = format!("{}{}.png", BASE_BACKGROUND_PATH, self.background_id);
            if !ResourceLoader::singleton().exists(&background_path) {
                godot_error!(
                    "[{}] Could not find background [{}]",
                    self._i._id,
                    self.background_id
                );
                return false;
            }

            if !self
                .segments
                .iter()
                .enumerate()
                .all(|(segment_idx, segment)| segment.validate(segment_idx, &self._i._id, db))
            {
                godot_error!("[{}] Chapter contains an invalid segment!", self._i._id);
                return false;
            }

            if self.next_chapter.is_empty()
                || (self.next_chapter.to_string() != NEXT_CHAPTER_MAIN_MENU
                    && !db.chapters.contains_key(&self.next_chapter))
            {
                godot_error!(
                    "[{}] Next chapter [{}] could not be found!",
                    self._i._id,
                    self.next_chapter
                );
                return false;
            }

            true
        }
    }

    impl ChapterSegment {
        pub(self) fn validate(
            &self,
            segment_idx: usize,
            chapter_key: &ChapterKey,
            db: &DbConnector,
        ) -> bool {
            match self {
                ChapterSegment::Dialogue(dialogue_config) => {
                    dialogue_config.validate(segment_idx, chapter_key, db)
                }
                ChapterSegment::Camp(camp_config) => {
                    camp_config.validate(segment_idx, chapter_key, db)
                }
                ChapterSegment::Battle(battle_config) => {
                    battle_config.validate(segment_idx, chapter_key, db)
                }
            }
        }
    }
}
