use crate::traits::ToVariantArray;

use super::{DbId, DbTable, IdColumn};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

mod combat_flow_modifier;
mod combat_stat_modifier;
mod growth_modifier;
mod health;
mod stat_modifier;

pub(crate) use combat_flow_modifier::*;
pub(crate) use combat_stat_modifier::*;
pub(crate) use growth_modifier::*;
pub(crate) use health::*;
pub(crate) use stat_modifier::*;

pub(crate) type EffectId = DbId;

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum UnitStat {
    Htp = 0,
    Str = 1,
    Mag = 2,
    Def = 3,
    Spt = 4,
    Agi = 5,
    Dex = 6,
    Mov = 7,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum UnitCombatStat {
    Hit = 0,
    Avo = 1,
    Crit = 2,
    Dodge = 3,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
#[serde(tag = "type", content = "params")]
pub(crate) enum EffectVariant {
    /// Used for effects that only have child effects
    Parent(Vec<EffectId>),
    /// Used for effcts that fill/drain htp/valor from a unit
    Health(HealthEffect),
    /// Modifiers on unit's base states (e.g. +1 str, -1 def)
    StatModifier(StatEffect),
    /// Modifiers on unit's combat stats (e.g. +15 avo, -10 crit)
    CombatStatModifier(CombatStatEffect),
    /// Effects that alter the normal flow of combat
    /// (e.g. attacking first, not allowing second wind etc.)
    CombatFlowModifier(CombatFlowEffect),
    /// Effects that alter the units growth rates
    GrowthModifier(GrowthEffect),
}

impl GodotConvert for EffectVariant {
    type Via = Dictionary;
}

impl ToGodot for EffectVariant {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        match self {
            EffectVariant::Parent(child_effects) => {
                dict! {
                    "type": 0,
                    "params": child_effects.to_variant_array(),
                }
            }
            EffectVariant::Health(health_effect) => {
                dict! {
                    "type": 1,
                    "params": health_effect.to_godot(),
                }
            }
            EffectVariant::StatModifier(stat_effect) => {
                dict! {
                    "type": 2,
                    "params": stat_effect.to_godot(),
                }
            }
            EffectVariant::CombatStatModifier(combat_stat_effect) => {
                dict! {
                    "type": 3,
                    "params": combat_stat_effect.to_godot(),
                }
            }
            EffectVariant::CombatFlowModifier(combat_flow_effect) => {
                dict! {
                    "type": 4,
                    "params": combat_flow_effect.to_godot(),
                }
            }
            EffectVariant::GrowthModifier(growth_effect) => {
                dict! {
                    "type": 5,
                    "params": growth_effect.to_godot(),
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EffectEntry {
    #[serde(flatten)]
    _i: IdColumn,
    #[serde(flatten)]
    variant: EffectVariant,
}

impl DbTable for EffectEntry {
    fn get_id(&self) -> DbId {
        self._i._id.clone()
    }
}

impl GodotConvert for EffectEntry {
    type Via = Dictionary;
}

impl ToGodot for EffectEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut effect_dict = dict! {
            "id": self._i._id.clone(),
        };

        effect_dict.extend_dictionary(&self.variant.to_godot(), true);

        effect_dict
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::{EffectEntry, EffectVariant};
    use crate::database::{DbConnector, validation::VerifyTable};

    use godot::{builtin::StringName, global::godot_error};

    impl VerifyTable for EffectEntry {
        fn validate(&self, db: &DbConnector) -> bool {
            if self._i._id.is_empty() {
                godot_error!("[{}] Invalid effect row in database!", self._i._id);
                return false;
            }

            if !self.variant.validate(&self._i._id, db) {
                return false;
            }

            true
        }
    }

    impl EffectVariant {
        fn validate(&self, effect_id: &StringName, db: &DbConnector) -> bool {
            match self {
                EffectVariant::Parent(child_effects) => {
                    for child_effect in child_effects {
                        if !db.effects.contains_key(child_effect) {
                            godot_error!("[{}] Invalid child effect: {}", effect_id, child_effect);
                            return false;
                        }
                    }
                }
                EffectVariant::Health(health_effect) => {
                    if !health_effect.validate(effect_id, db) {
                        return false;
                    }
                }
                EffectVariant::StatModifier(stat_effect) => {
                    if !stat_effect.validate(effect_id, db) {
                        return false;
                    }
                }
                EffectVariant::CombatStatModifier(combat_stat_effect) => {
                    if !combat_stat_effect.validate(effect_id, db) {
                        return false;
                    }
                }
                EffectVariant::CombatFlowModifier(combat_flow_effect) => {
                    if !combat_flow_effect.validate(effect_id, db) {
                        return false;
                    }
                }
                EffectVariant::GrowthModifier(growth_effect) => {
                    if !growth_effect.validate(effect_id, db) {
                        return false;
                    }
                }
            }

            true
        }
    }
}
