use super::{DbId, DbTable, IdColumn, NameDescColumns, effect::EffectId};

use godot::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) type SkillId = DbId;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum SkillTrigger {
    MapStart = 0,
    TurnStart = 1,
    TurnEnd = 2,
    CombatStart = 3,
    CombatEnd = 4,
    Passive = 5,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum SkillTriggerCondition {
    #[default]
    None = 0,
    HtpValue = 1,
    AdjacentAlliesCount = 2,
    AdjacentEnemiesCount = 3,
    MovUsed = 4,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum SlotType {
    #[default]
    Personal = 0,
    General = 1,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SkillEntry {
    #[serde(flatten)]
    _i: IdColumn,
    #[serde(flatten)]
    _n: NameDescColumns,
    effect_id: EffectId,
    trigger: SkillTrigger,
    #[serde(default)]
    trigger_condition: SkillTriggerCondition,
    #[serde(default)]
    slot_type: SlotType,
    #[serde(default)]
    inheritable: bool,
}

impl DbTable for SkillEntry {
    fn get_id(&self) -> DbId {
        self._i._id.clone()
    }
}

impl GodotConvert for SkillEntry {
    type Via = Dictionary;
}

impl ToGodot for SkillEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "id": self._i._id.clone(),
            "name": self._n.name.clone(),
            "description": self._n.description.clone(),
            "effect_id": self.effect_id.clone(),
            "trigger": self.trigger as u8,
            "trigger_condition": self.trigger_condition as u8,
            "slot_type": self.slot_type as u8,
            "inheritable": self.inheritable,
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::SkillEntry;
    use crate::database::{DbConnector, validation::VerifyTable};

    use godot::global::godot_error;

    impl VerifyTable for SkillEntry {
        fn validate(&self, db: &DbConnector) -> bool {
            if self._i._id.is_empty() || self._n.is_empty() {
                godot_error!("[{}] Invalid skill row in database!", self._i._id);
                return false;
            }

            if !db.effects.contains_key(&self.effect_id) {
                godot_error!(
                    "[{}] Skill effect_id [{}] not found in database!",
                    self._i._id,
                    self.effect_id
                );
                return false;
            }

            true
        }
    }
}
