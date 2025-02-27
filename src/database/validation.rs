use super::{DbConnector, DbId};

use godot::prelude::*;
use std::collections::{HashMap, HashSet};

pub(super) trait VerifyTable {
    fn validate(&self, db: &DbConnector) -> bool;
}

impl DbConnector {
    pub(crate) fn verify(&self) -> bool {
        self.ensure_all_ids_unique() && self.ensure_all_rows_valid()
    }

    fn ensure_all_ids_unique(&self) -> bool {
        DbConnector::ensure_all_ids_unique_for("armies", &self.armies)
            && DbConnector::ensure_all_ids_unique_for("chapters", &self.chapters)
            && DbConnector::ensure_all_ids_unique_for("effects", &self.effects)
            && DbConnector::ensure_all_ids_unique_for("inventory", &self.inventory)
            && DbConnector::ensure_all_ids_unique_for("kits", &self.kits)
            && DbConnector::ensure_all_ids_unique_for("roles", &self.roles)
            && DbConnector::ensure_all_ids_unique_for("skills", &self.skills)
            && DbConnector::ensure_all_ids_unique_for("units", &self.units)
            && DbConnector::ensure_all_ids_unique_for("personalities", &self.personalities)
    }

    fn ensure_all_ids_unique_for<T>(table_name: &str, table: &HashMap<DbId, T>) -> bool {
        let mut seen_ids = HashSet::with_capacity(table.len());

        for key in table.keys() {
            if !seen_ids.insert(key) {
                godot_error!("Repeated row identifiers found in table: {}", table_name);
                return false;
            }
        }

        true
    }

    fn ensure_all_rows_valid(&self) -> bool {
        DbConnector::ensure_all_rows_valid_for(&self.armies, self)
            && DbConnector::ensure_all_rows_valid_for(&self.chapters, self)
            && DbConnector::ensure_all_rows_valid_for(&self.effects, self)
            && DbConnector::ensure_all_rows_valid_for(&self.inventory, self)
            && DbConnector::ensure_all_rows_valid_for(&self.kits, self)
            && DbConnector::ensure_all_rows_valid_for(&self.roles, self)
            && DbConnector::ensure_all_rows_valid_for(&self.skills, self)
            && DbConnector::ensure_all_rows_valid_for(&self.units, self)
            && DbConnector::ensure_all_rows_valid_for(&self.personalities, self)
    }

    fn ensure_all_rows_valid_for<T>(table: &HashMap<DbId, T>, connector: &Self) -> bool
    where
        T: VerifyTable,
    {
        table.iter().all(|(_, row)| row.validate(connector))
    }
}
