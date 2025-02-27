use super::EffectTarget;
use crate::database::effect::EffectId;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum SupportCategory {
    Staff = 0,
    Flute = 1,
    Maracas = 2,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) struct SupportEntry {
    pub(crate) category: SupportCategory,
    pub(crate) effect_id: EffectId,
    #[serde(flatten)]
    pub(crate) effect_target: EffectTarget,
    #[serde(default)]
    pub(crate) hit_mod: i8,
    #[serde(default)]
    pub(crate) avo_mod: i8,
    #[serde(default)]
    pub(crate) dodge_mod: i8,
    #[serde(default)]
    pub(crate) required_str: u8,
    #[serde(default)]
    pub(crate) required_mag: u8,
    #[serde(default)]
    pub(crate) required_dex: u8,
}

impl GodotConvert for SupportEntry {
    type Via = Dictionary;
}

impl ToGodot for SupportEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut support_dict = dict! {
            "category": self.category as u8,
            "effect_id": self.effect_id.clone(),
            "hit_mod": self.hit_mod,
            "avo_mod": self.avo_mod,
            "dodge_mod": self.dodge_mod,
            "required_str": self.required_str,
            "required_mag": self.required_mag,
            "required_dex": self.required_dex,
        };

        support_dict.extend_dictionary(&self.effect_target.to_godot(), true);

        support_dict
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::SupportEntry;
    use crate::database::DbConnector;

    use godot::global::godot_error;

    impl SupportEntry {
        pub(crate) fn validate(&self, db: &DbConnector) -> bool {
            if !self.effect_target.validate() {
                return false;
            }

            if self.required_str == 0 && self.required_mag == 0 && self.required_dex == 0 {
                godot_error!("At least one required attribute (str, mag, dex) must be non-zero!");
                return false;
            }

            if self.hit_mod < -100 || self.hit_mod > 100 {
                godot_error!("Invalid 'hit_mod' for support! Must be between -100 and 100.");
                return false;
            }

            if self.avo_mod < -100 || self.avo_mod > 100 {
                godot_error!("Invalid 'avo_mod' for support! Must be between -100 and 100.");
                return false;
            }

            if self.dodge_mod < -100 || self.dodge_mod > 100 {
                godot_error!("Invalid 'dodge_mod' for weapon! Must be between -100 and 100.");
                return false;
            }

            if !self.effect_id.is_empty() && !db.effects.contains_key(&self.effect_id) {
                godot_error!(
                    "Support effect_id [{}] not found in database!",
                    self.effect_id
                );
                return false;
            }

            true
        }
    }
}
