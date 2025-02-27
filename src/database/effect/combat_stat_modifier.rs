use super::UnitCombatStat;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CombatStatEffect {
    pub stat: UnitCombatStat,
    pub amount: i8,
}

impl GodotConvert for CombatStatEffect {
    type Via = Dictionary;
}

impl ToGodot for CombatStatEffect {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::ToVia<'_> {
        dict! {
            "stat": self.stat as u8,
            "amount": self.amount,
        }
    }
}

mod verify {
    use super::CombatStatEffect;
    use crate::database::DbConnector;

    use godot::{builtin::StringName, global::godot_error};

    impl CombatStatEffect {
        pub(crate) fn validate(&self, effect_id: &StringName, _: &DbConnector) -> bool {
            if self.amount == 0 {
                godot_error!(
                    "[{}] Invalid CombatStatEffect! 'amount' should be different than zero!",
                    effect_id
                );
                return false;
            }

            if self.amount < -50 || self.amount > 50 {
                godot_error!(
                    "[{}] Invalid CombatStatEffect! 'amount' should be within the valid range (-50 to 50)!",
                    effect_id
                );
                return false;
            }

            true
        }
    }
}
