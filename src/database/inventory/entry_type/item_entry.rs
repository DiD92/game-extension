use super::EffectTarget;
use crate::database::effect::EffectId;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ConsumableEntry {
    pub(crate) effect_id: EffectId,
    #[serde(flatten)]
    pub(crate) effect_target: EffectTarget,
}

impl GodotConvert for ConsumableEntry {
    type Via = Dictionary;
}

impl ToGodot for ConsumableEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut target_dict = self.effect_target.to_godot();
        target_dict.set("effect_id", self.effect_id.clone());
        target_dict
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"), tag = "category")]
pub(crate) enum ItemEntry {
    Key,
    Consumable(ConsumableEntry),
}

impl GodotConvert for ItemEntry {
    type Via = Dictionary;
}

impl ToGodot for ItemEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        match self {
            ItemEntry::Key => dict! {
                "category": "key",
            },
            ItemEntry::Consumable(entry) => {
                let mut consumable_entry = entry.to_godot();
                consumable_entry.set("category", "consumable");
                consumable_entry
            }
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::{ConsumableEntry, ItemEntry};
    use crate::database::DbConnector;

    use godot::global::godot_error;

    impl ItemEntry {
        pub(crate) fn validate(&self, db: &DbConnector) -> bool {
            match self {
                ItemEntry::Key => true,
                ItemEntry::Consumable(entry) => entry.validate(db),
            }
        }
    }

    impl ConsumableEntry {
        pub(crate) fn validate(&self, db: &DbConnector) -> bool {
            if !db.effects.contains_key(&self.effect_id) {
                godot_error!(
                    "Consumable effect_id [{}] not found in database!",
                    self.effect_id
                );
                return false;
            }

            if !self.effect_target.validate() {
                return false;
            }

            true
        }
    }
}
