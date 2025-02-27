use chrono::TimeZone;
use godot::prelude::*;
use rust_extensions_macros::ToGodotDictionary;
use serde::{Deserialize, Serialize};

mod barracks_state;
mod game_state;

pub const SAVE_SLOTS: usize = 5;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum PointType {
    Chapter = 0,
    Battle = 1,
    Manual = 2,
}

impl GodotConvert for PointType {
    type Via = u8;
}

impl ToGodot for PointType {
    type ToVia<'v> = u8;

    fn to_godot(&self) -> Self::ToVia<'_> {
        *self as u8
    }
}

impl FromGodot for PointType {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        match via {
            0 => Ok(Self::Chapter),
            1 => Ok(Self::Battle),
            2 => Ok(Self::Manual),
            other => Err(ConvertError::new(format!(
                "Unknown PointType value [{}]!",
                other
            ))),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, ToGodotDictionary)]
pub(super) struct SlotPoint {
    game_state: game_state::GameState,
    player_barracks: barracks_state::BarracksState,
    rand_state: i64,
}

impl SlotPoint {
    fn update_data(&mut self, update_data: Dictionary) {
        if let Some(data) = update_data.get("game_state") {
            self.game_state.update_data(Dictionary::from_variant(&data));
        }

        if let Some(data) = update_data.get("player_barracks") {
            self.player_barracks
                .update_data(Dictionary::from_variant(&data));
        }

        if let Some(data) = update_data.get("rand_state") {
            self.rand_state = i64::from_variant(&data);
        }
    }
}

impl FromGodot for SlotPoint {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            game_state: game_state::GameState::from_variant(&via.at("game_state")),
            player_barracks: barracks_state::BarracksState::from_variant(
                &via.at("player_barracks"),
            ),
            rand_state: i64::from_variant(&via.at("rand_state")),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub(super) struct SaveSlot {
    chapter: Option<SlotPoint>,
    battle: Option<SlotPoint>,
    manual: Option<SlotPoint>,
    // Represents a unix epoch
    saved_at: i64,
}

impl SaveSlot {
    pub(super) fn overwrite(&mut self, data: Dictionary, at: PointType) {
        let maybe_save_point = match at {
            PointType::Chapter => &mut self.chapter,
            PointType::Battle => &mut self.battle,
            PointType::Manual => &mut self.manual,
        };

        if let Some(save_point) = maybe_save_point {
            save_point.update_data(data);
        } else {
            *maybe_save_point = Some(SlotPoint::from_variant(&data.to_variant()));
        }

        self.saved_at = Self::get_current_epoch_secs();
    }

    pub(super) fn get_at(&self, at: PointType) -> Dictionary {
        let maybe_save_point = match at {
            PointType::Chapter => &self.chapter,
            PointType::Battle => &self.battle,
            PointType::Manual => &self.manual,
        };

        maybe_save_point
            .as_ref()
            .map(|point| point.to_godot())
            .unwrap_or_default()
    }

    pub(super) fn get_summary(&self) -> Dictionary {
        let mut dict = Dictionary::new();

        if let Some(slot_point) = self.manual.as_ref() {
            dict.set(
                "chapter_key",
                slot_point.game_state.chapter_state.key.clone(),
            );

            let state_summary = slot_point.game_state.chapter_state.get_summary();

            dict.extend(state_summary.iter_shared());
            if let Some(date_time) = chrono::offset::Local
                .timestamp_opt(self.saved_at, 0)
                .latest()
            {
                dict.set("saved_at", date_time.to_rfc3339());
            } else {
                godot_warn!(
                    "Failed to convert epoch timestamp [{}] to datetime!",
                    self.saved_at
                );
                dict.set(
                    "saved_at",
                    chrono::DateTime::<chrono::Local>::default().to_rfc3339(),
                );
            }
        }

        dict
    }

    pub(super) fn has_data(&self, at: PointType) -> bool {
        match at {
            PointType::Chapter => self.chapter.is_some(),
            PointType::Battle => self.battle.is_some(),
            PointType::Manual => self.manual.is_some(),
        }
    }

    fn get_current_epoch_secs() -> i64 {
        chrono::offset::Local::now().timestamp()
    }
}

impl Clone for SaveSlot {
    fn clone(&self) -> Self {
        Self {
            chapter: self.chapter.clone(),
            battle: self.battle.clone(),
            manual: self.manual.clone(),
            saved_at: SaveSlot::get_current_epoch_secs(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub(super) struct SaveSlots {
    pub(super) auto: SaveSlot,
    pub(super) idx: [Option<SaveSlot>; SAVE_SLOTS],
}
