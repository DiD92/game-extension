use godot::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) type DbId = StringName;

pub(crate) trait DbTable {
    fn get_id(&self) -> DbId;
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct IdColumn {
    pub(crate) _id: DbId,
}
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct NameDescColumns {
    pub(crate) name: StringName,
    pub(crate) description: StringName,
}

#[cfg(feature = "verify_database")]
impl NameDescColumns {
    pub(crate) fn is_empty(&self) -> bool {
        self.name.is_empty() || self.description.is_empty()
    }
}
