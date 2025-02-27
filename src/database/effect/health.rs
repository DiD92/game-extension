use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub(crate) enum HealthTarget {
    Htp = 0,
    Valor = 1,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub(crate) struct HealthEffect {
    pub(crate) target: HealthTarget,
    pub(crate) power: i8,
}

impl GodotConvert for HealthEffect {
    type Via = Dictionary;
}

impl ToGodot for HealthEffect {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::ToVia<'_> {
        dict! {
            "target": self.target as u8,
            "power": self.power,
        }
    }
}

mod verify {
    use super::HealthEffect;
    use crate::database::DbConnector;

    use godot::{builtin::StringName, global::godot_error};

    impl HealthEffect {
        pub(crate) fn validate(&self, effect_id: &StringName, _: &DbConnector) -> bool {
            if self.power == 0 {
                godot_error!(
                    "[{}] Invalid HealthEffect! 'power' should be different than zero!",
                    effect_id
                );
                return false;
            }

            if self.power < -20 || self.power > 20 {
                godot_error!(
                    "[{}] Invalid HealthEffect! 'power' should be within the valid range (-20 to 20)!",
                    effect_id
                );
                return false;
            }

            true
        }
    }
}
