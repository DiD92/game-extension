use crate::database::chapter::Vector2u8;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

mod item_entry;
mod support_entry;
mod weapon_entry;

pub(crate) use item_entry::*;
pub(crate) use support_entry::*;
pub(crate) use weapon_entry::*;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    rename_all(deserialize = "snake_case"),
    tag = "effect_target",
    content = "effect_range"
)]
pub(crate) enum EffectTarget {
    Oneself,
    Ally(Vector2u8),
    Enemy(Vector2u8),
    Allies(Vector2u8),
    Enemies(Vector2u8),
    All(Vector2u8),
}

impl GodotConvert for EffectTarget {
    type Via = Dictionary;
}

impl ToGodot for EffectTarget {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        match self {
            EffectTarget::Oneself => dict! {
                "target": 0,
            },
            EffectTarget::Ally(effect_range) => dict! {
                "target": 1,
                "range": effect_range.to_godot(),
            },
            EffectTarget::Enemy(effect_range) => dict! {
                "target": 2,
                "range": effect_range.to_godot(),
            },
            EffectTarget::Allies(effect_range) => dict! {
                "target": 3,
                "range": effect_range.to_godot(),
            },
            EffectTarget::Enemies(effect_range) => dict! {
                "target": 4,
                "range": effect_range.to_godot(),
            },
            EffectTarget::All(effect_range) => dict! {
                "target": 5,
                "range": effect_range.to_godot(),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SlotType {
    Physical = 0,
    Magical = 1,
    Support = 2,
    #[default]
    Item = 3,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"), tag = "type")]
pub(crate) enum EntryVariant {
    Weapon(WeaponEntry),
    Support(SupportEntry),
    Item(ItemEntry),
}

impl EntryVariant {
    pub(crate) fn get_range(&self) -> Option<Vector2i> {
        match self {
            EntryVariant::Weapon(w) => Some(w.range.to_godot()),
            EntryVariant::Support(s) => match s.effect_target {
                EffectTarget::Ally(effect_range)
                | EffectTarget::Enemy(effect_range)
                | EffectTarget::Allies(effect_range)
                | EffectTarget::Enemies(effect_range)
                | EffectTarget::All(effect_range) => Some(effect_range.to_godot()),
                _ => None,
            },
            EntryVariant::Item(_) => None,
        }
    }

    pub(crate) fn get_slot_type(&self) -> SlotType {
        match self {
            EntryVariant::Weapon(weapon_entry) => match weapon_entry.slot_category {
                WeaponSlotCategory::Physical => SlotType::Physical,
                WeaponSlotCategory::Magical => SlotType::Magical,
            },
            EntryVariant::Support(_) => SlotType::Support,
            EntryVariant::Item(_) => SlotType::Item,
        }
    }
}

impl GodotConvert for EntryVariant {
    type Via = Dictionary;
}

impl ToGodot for EntryVariant {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut entry_dict = Dictionary::new();

        match self {
            EntryVariant::Weapon(entry) => {
                entry_dict.set("type", 0);
                entry_dict.extend_dictionary(&entry.to_godot(), true);
            }
            EntryVariant::Support(entry) => {
                entry_dict.set("type", 1);
                entry_dict.extend_dictionary(&entry.to_godot(), true);
            }
            EntryVariant::Item(entry) => {
                entry_dict.set("type", 2);
                entry_dict.extend_dictionary(&entry.to_godot(), true);
            }
        }

        entry_dict
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::{EffectTarget, EntryVariant};
    use crate::database::DbConnector;

    use godot::global::godot_error;

    impl EntryVariant {
        pub(crate) fn validate(&self, db: &DbConnector) -> bool {
            match self {
                EntryVariant::Weapon(weapon_entry) => weapon_entry.validate(db),
                EntryVariant::Support(support_entry) => support_entry.validate(db),
                EntryVariant::Item(item_entry) => item_entry.validate(db),
            }
        }
    }

    impl EffectTarget {
        pub(crate) fn validate(&self) -> bool {
            match self {
                EffectTarget::Ally(effect_range)
                | EffectTarget::Enemy(effect_range)
                | EffectTarget::Allies(effect_range)
                | EffectTarget::Enemies(effect_range)
                | EffectTarget::All(effect_range) => {
                    if effect_range.x > effect_range.y {
                        godot_error!("Invalid range [{:?}] for effect!", effect_range);
                        false
                    } else {
                        true
                    }
                }
                _ => true,
            }
        }
    }
}
