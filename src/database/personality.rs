use super::{DbId, DbTable, IdColumn};

use godot::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::rust::maps_duplicate_key_is_error;
use std::collections::HashMap;

pub(crate) type PersonalityId = DbId;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum BehaviourKey {
    /// The unit's key behaviour when it has low health.
    LowHtp = 0,
    /// The unit's key behaviour when it has low valor.
    LowValor = 1,
    /// The unit's key behaviour
    /// when more enemies than allies are close to it.
    EnemiesClose = 2,
    /// The unit's key behaviour when more
    /// allies than enemies are close to it.
    AlliesClose = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum MovementBehaviour {
    /// The unit will not move.
    Stationary = 0,
    /// The unit will stay in place or move towards its
    /// initial position.
    Defend = 1,
    /// The unit will move towards the enemies.
    Vanguard = 2,
    /// The unit will move away from the enemies.
    Evade = 3,
    /// The unit will move according to the objective.
    Tactician = 4,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum ActionBehaviour {
    /// The unit will do nothing.
    DoNothing = 0,
    /// The units will attack the closest enemy.
    AttackCloserEnemy = 1,
    /// The unit will attack the enemy to which it deals the most damage.
    AttackWeakerEnemy = 2,
    /// The unit will attack the enemy while minimizing the damage it takes.
    AttackMinimizingDamage = 3,
    /// The unit will heal the allies with the lowest health.
    HealAllies = 4,
    /// The unit will heal itself.
    HealOneself = 5,
    /// The unit will buff the most allies possible.
    BuffAllies = 6,
    /// The unit will buff itself.
    BuffOneself = 7,
    /// The unit will debuff the most enemies possible.
    DebuffEnemies = 8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct UnitBehaviour {
    /// The priority of the behaviour. `UnitBehaviour` entries with **higher
    /// priority will take precedence** over those with lower priority.
    #[serde(default)]
    pub(crate) priority: u8,
    pub(crate) movement: MovementBehaviour,
    pub(crate) action: ActionBehaviour,
}

impl GodotConvert for UnitBehaviour {
    type Via = Dictionary;
}

impl ToGodot for UnitBehaviour {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::ToVia<'_> {
        dict! {
            "movement": self.movement as u8,
            "action": self.action as u8,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PersonalityEntry {
    #[serde(flatten)]
    _i: IdColumn,
    default: UnitBehaviour,
    #[serde(default, with = "maps_duplicate_key_is_error")]
    conditional: HashMap<BehaviourKey, UnitBehaviour>,
}

impl DbTable for PersonalityEntry {
    fn get_id(&self) -> DbId {
        self._i._id.clone()
    }
}

impl GodotConvert for PersonalityEntry {
    type Via = Dictionary;
}

impl ToGodot for PersonalityEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::ToVia<'_> {
        let conditional_dict = self
            .conditional
            .iter()
            .map(|(k, v)| (*k as u8, v.to_godot()))
            .collect::<Dictionary>();

        dict! {
            "id": self._i._id.clone(),
            "default": self.default.to_godot(),
            "conditional": conditional_dict,
        }
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::PersonalityEntry;
    use crate::database::{DbConnector, validation::VerifyTable};

    use godot::global::godot_error;
    use std::collections::HashSet;

    impl VerifyTable for PersonalityEntry {
        fn validate(&self, _: &DbConnector) -> bool {
            if self._i._id.is_empty() {
                godot_error!("[{}] Invalid personality row in database!", self._i._id);
                return false;
            }

            let mut visited_priorities = HashSet::new();

            for behaviour in self.conditional.values() {
                if visited_priorities.contains(&behaviour.priority) {
                    godot_error!(
                        "[{}] Duplicate priority [{}] in personality conditional!",
                        self._i._id,
                        behaviour.priority
                    );
                    return false;
                }

                visited_priorities.insert(behaviour.priority);
            }

            true
        }
    }
}
