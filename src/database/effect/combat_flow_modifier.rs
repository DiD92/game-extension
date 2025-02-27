use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum CombatFlowEffect {
    AttackFirst = 0,
    NoSecondWind = 1,
}

impl GodotConvert for CombatFlowEffect {
    type Via = u8;
}

impl ToGodot for CombatFlowEffect {
    type ToVia<'v> = u8;

    fn to_godot(&self) -> Self::ToVia<'_> {
        *self as u8
    }
}

mod verify {
    use super::CombatFlowEffect;
    use crate::database::DbConnector;

    use godot::builtin::StringName;

    impl CombatFlowEffect {
        pub(crate) fn validate(&self, _: &StringName, _: &DbConnector) -> bool {
            true
        }
    }
}
