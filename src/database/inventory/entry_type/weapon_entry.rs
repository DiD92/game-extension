use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database::chapter::Vector2u8;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum WeaponCategory {
    Sword = 0,
    Lance = 1,
    Axe = 2,
    Bow = 3,
    Ballista = 4,
    Fist = 5,
    Bracelet = 6,
    Ring = 7,
    Circlet = 8,
    Fang = 9,
    Breath = 10,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum WeaponColor {
    Red = 0,
    Blue = 1,
    Green = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum WeaponDamageType {
    Physical = 0,
    Magical = 1,
    Piercing = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum WeaponSlotCategory {
    Physical = 0,
    Magical = 1,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) struct WeaponEntry {
    pub(crate) category: WeaponCategory,
    pub(crate) color: WeaponColor,
    pub(crate) damage_type: WeaponDamageType,
    pub(crate) slot_category: WeaponSlotCategory,
    pub(crate) range: Vector2u8,
    pub(crate) power: u8,
    #[serde(default)]
    pub(crate) hit_mod: i8,
    #[serde(default)]
    pub(crate) avo_mod: i8,
    #[serde(default)]
    pub(crate) crit_mod: i8,
    #[serde(default)]
    pub(crate) dodge_mod: i8,
    #[serde(default)]
    pub(crate) required_str: u8,
    #[serde(default)]
    pub(crate) required_mag: u8,
    #[serde(default)]
    pub(crate) required_dex: u8,
}

impl GodotConvert for WeaponEntry {
    type Via = Dictionary;
}

impl ToGodot for WeaponEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut entry = Dictionary::new();

        entry.set("category", self.category as u8);
        entry.set("color", self.color as u8);
        entry.set("damage_type", self.damage_type as u8);
        entry.set("slot_category", self.slot_category as u8);

        entry.set("range", self.range.to_godot());

        entry.set("power", self.power);
        entry.set("hit_mod", self.hit_mod);
        entry.set("avo_mod", self.avo_mod);
        entry.set("crit_mod", self.crit_mod);
        entry.set("dodge_mod", self.dodge_mod);

        entry.set("required_str", self.required_str);
        entry.set("required_mag", self.required_mag);
        entry.set("required_dex", self.required_dex);

        entry
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::WeaponEntry;
    use crate::database::DbConnector;

    use godot::global::godot_error;

    impl WeaponEntry {
        pub(crate) fn validate(&self, _db: &DbConnector) -> bool {
            if self.power == 0 {
                godot_error!("Weapon power cannot be zero!");
                return false;
            }

            if self.required_str == 0 && self.required_mag == 0 && self.required_dex == 0 {
                godot_error!("At least one required attribute (str, mag, dex) must be non-zero!");
                return false;
            }

            if self.hit_mod < -100 || self.hit_mod > 100 {
                godot_error!("Invalid 'hit_mod' for weapon! Must be between -100 and 100.");
                return false;
            }

            if self.avo_mod < -100 || self.avo_mod > 100 {
                godot_error!("Invalid 'avo_mod' for weapon! Must be between -100 and 100.");
                return false;
            }

            if self.crit_mod < -100 || self.crit_mod > 100 {
                godot_error!("Invalid 'crit_mod' for weapon! Must be between -100 and 100.");
                return false;
            }

            if self.dodge_mod < -100 || self.dodge_mod > 100 {
                godot_error!("Invalid 'dodge_mod' for weapon! Must be between -100 and 100.");
                return false;
            }

            if self.range.x < 1 {
                godot_error!("Invalid min range [{}] for weapon!", self.range);
                return false;
            }

            if self.range.y < self.range.x {
                godot_error!("Invalid range [{}] for weapon!", self.range);
                return false;
            }

            if self.range.y > 10 {
                godot_error!("Invalid max range [{}] for weapon!", self.range);
                return false;
            }

            true
        }
    }
}
