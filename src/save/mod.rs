#[cfg(feature = "save_bin")]
use bitcode::{deserialize, serialize};
#[cfg(feature = "save_json")]
use serde_json::{from_slice as deserialize, to_vec_pretty as serialize};

use godot::prelude::*;
use std::vec::Vec;

pub(crate) mod slot;

/// Contains the save slots state. There is one auto slot and five indexed slots.
/// Data is saved by default at the auto slot, and then can be copied to an indexed slot.
///
/// The general structure of a **non-empty slot point** is the following:
///```
/// {
///     game_state: {
///         story_flags: {<flag_id>: <flag_value>},
///         character_flags: {<char_id>: <bool>},
///         chapter_state: {
///             key: <chapter_key>,
///             next_key: <chapter_key>,
///             current_segment_idx: <int>,
///             current_segment: {
///                 type: <segment_type_enum>,
///                 params: <segment_type_enum_params>,
///             }
///         }
///     },
///     player_barracks: {
///         item_idx_gen: <int>,
///         unit_idx_gen: <int>,
///         gold_amt: <int>,
///         inventory: {<entry_idx>: <inventory_entry>},
///         roles: {<unit_id>: {<role_id>: <role_entry>}},
///         kits: {<unit_id>: [<kit_id>]},
///         skills: {<unit_id>: [<skill_id>]},
///         units: {<unit_id>: <unit_data>},
///     },
/// }
///```
/// When overwriting any slot this is the format expected for the input data.
#[derive(GodotClass)]
struct SaveManager {
    slots: slot::SaveSlots,
}

#[godot_api]
impl IRefCounted for SaveManager {
    fn init(_base: Base<RefCounted>) -> Self {
        Self {
            slots: slot::SaveSlots::default(),
        }
    }
}

#[godot_api]
impl SaveManager {
    /// Overwrites the state from the data found at `file_path`.
    #[cfg(any(feature = "save_json", feature = "save_bin"))]
    #[func]
    fn load(&mut self, file_path: String) -> bool {
        let path = std::path::Path::new(&file_path);

        godot_print!(
            "[RustExtensions] Trying to load save file at [{}]",
            path.display()
        );

        match std::fs::read(path) {
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                godot_warn!("Save file could not be found, trying to create new empty file...");
                if Self::try_initialize_empty_save_file_at(path) {
                    self.load(file_path)
                } else {
                    false
                }
            }
            Err(err) => {
                godot_error!("Save file could not be read, reason: {}", err);
                false
            }
            Ok(file_contents) => match deserialize::<slot::SaveSlots>(&file_contents) {
                Ok(save_slots) => {
                    self.slots = save_slots;

                    true
                }
                Err(err) => {
                    godot_error!("Failed to load save data, reason: {}", err);
                    false
                }
            },
        }
    }

    #[cfg(any(feature = "save_json", feature = "save_bin"))]
    /// Persists the state to `file_path`.
    #[func]
    fn persist_at(&mut self, file_path: String) -> bool {
        let path = std::path::Path::new(&file_path);

        match serialize(&self.slots) {
            Ok(data) => match std::fs::write(path, data.as_slice()) {
                Ok(()) => {
                    godot_print!("[RustExtensions]: Persisted save data to [{}]", file_path);
                    true
                }
                Err(err) => {
                    godot_error!("Failed to persist save data, reason: {}", err);
                    false
                }
            },
            Err(err) => {
                godot_error!("Failed to serialize save state, reason: {}", err);
                false
            }
        }
    }

    /// Overwrites te state of the auto slot at point `at`.
    #[func]
    fn overwrite(&mut self, data: Dictionary, at: slot::PointType) {
        self.slots.auto.overwrite(data, at);
    }

    /// Clear te state of the auto slot.
    #[func]
    fn clear(&mut self) {
        self.slots.auto = slot::SaveSlot::default();
    }

    /// Copies the state of the auto slot to the `slot_idx` slot.
    #[func]
    fn copy_state_to(&mut self, slot_idx: u8) {
        if slot_idx < slot::SAVE_SLOTS as u8 {
            self.slots.idx[slot_idx as usize] = Some(self.slots.auto.clone());
        } else {
            godot_error!(
                "[RustExtensions]: Tried to copy save state to invalid slot idx [{}]!",
                slot_idx
            );
        }
    }

    /// Gets state from auto slot at point `at`.
    #[func]
    fn get_auto(&self, at: slot::PointType) -> Dictionary {
        self.slots.auto.get_at(at)
    }

    /// Gets state from indexed slot `slot_idx` at point `at`.
    #[func]
    fn get_idx(&self, slot_idx: u8, at: slot::PointType) -> Dictionary {
        if slot_idx < slot::SAVE_SLOTS as u8 {
            if let Some(slot) = self.slots.idx[slot_idx as usize].as_ref() {
                slot.get_at(at)
            } else {
                Dictionary::default()
            }
        } else {
            godot_warn!(
                "[RustExtensions]: Tried to get save state at invalid slot idx [{}]!",
                slot_idx
            );

            Dictionary::default()
        }
    }

    /// Get slot summary for `slot_idx`.
    ///
    /// Returns empty dictionary if the slot is empty.
    #[func]
    fn get_summary_for(&self, slot_idx: u8) -> Dictionary {
        if slot_idx < slot::SAVE_SLOTS as u8 {
            if let Some(slot) = self.slots.idx[slot_idx as usize].as_ref() {
                slot.get_summary()
            } else {
                Dictionary::default()
            }
        } else {
            godot_warn!(
                "[RustExtensions]: Tried to get save summary at invalid slot idx [{}]!",
                slot_idx
            );

            Dictionary::default()
        }
    }

    #[func]
    /// Returns true if auto slot has data at point `at`.
    fn has_data_at_auto(&self, at: slot::PointType) -> bool {
        self.slots.auto.has_data(at)
    }

    #[func]
    /// Returns true if slot `slot_idx` has data at point `at`.
    /// Will return false if `slot_idx` is out of bounds.
    fn has_data_at_idx(&self, slot_idx: u8, at: slot::PointType) -> bool {
        if slot_idx < slot::SAVE_SLOTS as u8 {
            if let Some(slot) = self.slots.idx[slot_idx as usize].as_ref() {
                slot.has_data(at)
            } else {
                false
            }
        } else {
            godot_warn!(
                "[RustExtensions]: Tried to get check slot at invalid slot idx [{}]!",
                slot_idx
            );
            false
        }
    }
}

#[cfg(any(feature = "save_json", feature = "save_bin"))]
impl SaveManager {
    fn try_initialize_empty_save_file_at(file_path: &std::path::Path) -> bool {
        match serialize(&slot::SaveSlots::default()) {
            Ok(data) => match std::fs::write(file_path, data) {
                Ok(()) => {
                    godot_print!(
                        "[RustExtensions]: Persisted initial save data to [{}]",
                        file_path.display()
                    );
                    true
                }
                Err(err) => {
                    godot_error!("Failed to persist initial save data, reason: {}", err);
                    false
                }
            },
            Err(err) => {
                godot_error!("Failed to serialize initial save state, reason: {}", err);
                false
            }
        }
    }
}
