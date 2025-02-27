use godot::prelude::*;
use std::{collections::hash_map::HashMap, io::ErrorKind::NotFound as FileNotFound, path::Path};

#[cfg(feature = "save_bin")]
use bitcode::deserialize;
use serde::de::DeserializeOwned;
#[cfg(feature = "save_json")]
use serde_json::from_slice as deserialize;

pub(crate) mod army;
pub(crate) mod chapter;
pub(crate) mod effect;
pub(crate) mod inventory;
pub(crate) mod kit;
pub(crate) mod personality;
pub(crate) mod role;
pub(crate) mod skill;
pub(crate) mod unit;

mod common;
mod godot_api;
#[cfg(feature = "verify_database")]
mod validation;

pub(crate) use common::*;

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub(crate) struct DbConnector {
    pub(crate) armies: HashMap<DbId, army::ArmyEntry>,
    pub(crate) chapters: HashMap<DbId, chapter::ChapterEntry>,
    pub(crate) effects: HashMap<DbId, effect::EffectEntry>,
    pub(crate) inventory: HashMap<DbId, inventory::InventoryEntry>,
    pub(crate) kits: HashMap<DbId, kit::KitEntry>,
    pub(crate) roles: HashMap<DbId, role::RoleEntry>,
    pub(crate) skills: HashMap<DbId, skill::SkillEntry>,
    pub(crate) units: HashMap<DbId, unit::UnitEntry>,
    pub(crate) personalities: HashMap<DbId, personality::PersonalityEntry>,
}

const TABLE_ARMIES: &str = "armies.json";
const TABLE_CHAPTERS: &str = "chapters.json";
const TABLE_EFFECTS: &str = "effects.json";
const TABLE_INVENTORY: &str = "inventory.json";
const TABLE_KITS: &str = "kits.json";
const TABLE_ROLES: &str = "roles.json";
const TABLE_SKILLS: &str = "skills.json";
const TABLE_UNITS: &str = "units.json";
const TABLE_PERSONALITIES: &str = "personalities.json";

impl DbConnector {
    fn try_load_table<T>(at: &Path) -> Option<Vec<T>>
    where
        T: DeserializeOwned + DbTable,
    {
        match std::fs::read(at) {
            Err(err) if err.kind() == FileNotFound => {
                godot_error!(
                    "Table file at {} could not be found!, reason: {}",
                    at.display(),
                    err
                );
                None
            }
            Err(err) => {
                godot_error!(
                    "Table file at {} could not be read!, reason: {}",
                    at.display(),
                    err
                );
                None
            }
            Ok(file_contents) => match deserialize::<Vec<T>>(&file_contents) {
                Ok(database_data) => Some(database_data),
                Err(err) => {
                    godot_error!(
                        "Failed to load database table at {}!, reason: {}",
                        at.display(),
                        err
                    );
                    None
                }
            },
        }
    }

    fn get_table_rows<T>(at: &Path) -> impl Iterator<Item = (DbId, T)>
    where
        T: DeserializeOwned + DbTable,
    {
        let map_closure = |entry: T| (entry.get_id(), entry);

        if let Some(rows) = DbConnector::try_load_table::<T>(at) {
            rows.into_iter().map(map_closure)
        } else {
            Vec::with_capacity(0).into_iter().map(map_closure)
        }
    }

    fn load_tables_from(&mut self, path: &std::path::Path) {
        if path.is_dir() {
            godot_print!(
                "[RustExtensions]: Loading database tables at [{}]",
                path.display()
            );

            self.armies
                .extend(DbConnector::get_table_rows(&path.join(TABLE_ARMIES)));
            self.chapters
                .extend(DbConnector::get_table_rows(&path.join(TABLE_CHAPTERS)));
            self.effects
                .extend(DbConnector::get_table_rows(&path.join(TABLE_EFFECTS)));
            self.inventory
                .extend(DbConnector::get_table_rows(&path.join(TABLE_INVENTORY)));
            self.kits
                .extend(DbConnector::get_table_rows(&path.join(TABLE_KITS)));
            self.roles
                .extend(DbConnector::get_table_rows(&path.join(TABLE_ROLES)));
            self.skills
                .extend(DbConnector::get_table_rows(&path.join(TABLE_SKILLS)));
            self.units
                .extend(DbConnector::get_table_rows(&path.join(TABLE_UNITS)));
            self.personalities
                .extend(DbConnector::get_table_rows(&path.join(TABLE_PERSONALITIES)));

            godot_print!("[RustExtensions]: Finished loading database!");
        } else {
            godot_error!(
                "Database directory {} is not a valid directory!",
                path.display()
            );
        }
    }
}
