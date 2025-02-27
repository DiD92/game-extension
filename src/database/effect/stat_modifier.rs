use super::UnitStat;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StatEffect {
    pub stat: UnitStat,
    pub amount: i8,
}

impl GodotConvert for StatEffect {
    type Via = Dictionary;
}

impl ToGodot for StatEffect {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::ToVia<'_> {
        dict! {
            "stat": self.stat as u8,
            "amount": self.amount,
        }
    }
}

mod verify {
    use super::StatEffect;
    use crate::database::DbConnector;

    use godot::{builtin::StringName, global::godot_error};

    impl StatEffect {
        pub(crate) fn validate(&self, effect_id: &StringName, _: &DbConnector) -> bool {
            if self.amount == 0 {
                godot_error!(
                    "[{}] Invalid StatEffect! 'amount' should be different than zero!",
                    effect_id
                );
                return false;
            }

            if self.amount < -20 || self.amount > 20 {
                godot_error!(
                    "[{}] Invalid StatEffect! 'amount' should be within the valid range (-20 to 20)!",
                    effect_id
                );
                return false;
            }

            true
        }
    }
}
