use super::UnitStat;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GrowthEffect {
    pub stat: UnitStat,
    pub amount: i8,
}

impl GodotConvert for GrowthEffect {
    type Via = Dictionary;
}

impl ToGodot for GrowthEffect {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::ToVia<'_> {
        dict! {
            "stat": self.stat as u8,
            "amount": self.amount,
        }
    }
}

mod verify {
    use super::GrowthEffect;
    use crate::database::DbConnector;

    use godot::{builtin::StringName, global::godot_error};

    impl GrowthEffect {
        pub(crate) fn validate(&self, effect_id: &StringName, _: &DbConnector) -> bool {
            if self.amount == 0 {
                godot_error!(
                    "[{}] Invalid GrowthEffect! 'amount' should be different than zero!",
                    effect_id
                );
                return false;
            }

            if self.amount < -35 || self.amount > 35 {
                godot_error!(
                    "[{}] Invalid GrowthEffect! 'amount' should be within the valid range (-35 to 35)!",
                    effect_id
                );
                return false;
            }

            true
        }
    }
}
