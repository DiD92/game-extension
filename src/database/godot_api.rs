use super::{DbConnector, DbId};

use godot::prelude::*;
use std::collections::HashMap;

#[godot_api]
#[cfg(any(feature = "save_json", feature = "save_bin"))]
impl DbConnector {
    #[func]
    fn load_database(&mut self, database_directory: String) {
        let db_directory_path = std::path::Path::new(&database_directory);
        self.load_tables_from(db_directory_path);
    }

    #[inline]
    fn get_from<T>(table: &HashMap<DbId, T>, id: &DbId) -> Dictionary
    where
        for<'v> T: ToGodot<ToVia<'v> = Dictionary> + 'v,
    {
        table.get(id).map(T::to_godot).unwrap_or_default()
    }

    #[inline]
    fn get_array_from<T>(table: &HashMap<DbId, T>) -> Array<Dictionary>
    where
        for<'v> T: ToGodot<ToVia<'v> = Dictionary> + 'v,
    {
        table.values().map(T::to_godot).collect()
    }

    #[func]
    pub(crate) fn get_army(&self, army_id: DbId) -> Dictionary {
        DbConnector::get_from(&self.armies, &army_id)
    }

    #[func]
    pub(crate) fn get_armies(&self) -> Array<Dictionary> {
        DbConnector::get_array_from(&self.armies)
    }

    #[func]
    pub(crate) fn has_army(&self, army_id: DbId) -> bool {
        self.armies.contains_key(&army_id)
    }

    #[func]
    pub(crate) fn get_chapter(&self, chapter_id: DbId) -> Dictionary {
        DbConnector::get_from(&self.chapters, &chapter_id)
    }

    #[func]
    pub(crate) fn get_chapters(&self) -> Array<Dictionary> {
        DbConnector::get_array_from(&self.chapters)
    }

    #[func]
    pub(crate) fn has_chapter(&self, chapter_id: DbId) -> bool {
        self.chapters.contains_key(&chapter_id)
    }

    #[func]
    pub(crate) fn get_effect(&self, effect_id: DbId) -> Dictionary {
        DbConnector::get_from(&self.effects, &effect_id)
    }

    #[func]
    pub(crate) fn get_effects(&self) -> Array<Dictionary> {
        DbConnector::get_array_from(&self.effects)
    }

    #[func]
    pub(crate) fn has_effect(&self, effect_id: DbId) -> bool {
        self.effects.contains_key(&effect_id)
    }

    #[func]
    pub(crate) fn get_inventory_entry(&self, item_id: DbId) -> Dictionary {
        DbConnector::get_from(&self.inventory, &item_id)
    }

    #[func]
    pub(crate) fn get_inventory_entries(&self) -> Array<Dictionary> {
        DbConnector::get_array_from(&self.inventory)
    }

    #[func]
    pub(crate) fn has_inventory_entry(&self, item_id: DbId) -> bool {
        self.inventory.contains_key(&item_id)
    }

    #[func]
    pub(crate) fn get_kit(&self, kit_id: DbId) -> Dictionary {
        DbConnector::get_from(&self.kits, &kit_id)
    }

    #[func]
    pub(crate) fn get_kits(&self) -> Array<Dictionary> {
        DbConnector::get_array_from(&self.kits)
    }

    #[func]
    pub(crate) fn has_kit(&self, kit_id: DbId) -> bool {
        self.kits.contains_key(&kit_id)
    }

    #[func]
    pub(crate) fn get_role(&self, role_id: DbId) -> Dictionary {
        DbConnector::get_from(&self.roles, &role_id)
    }

    #[func]
    pub(crate) fn get_roles(&self) -> Array<Dictionary> {
        DbConnector::get_array_from(&self.roles)
    }

    #[func]
    pub(crate) fn has_role(&self, role_id: DbId) -> bool {
        self.roles.contains_key(&role_id)
    }

    #[func]
    pub(crate) fn get_skill(&self, skill_id: DbId) -> Dictionary {
        DbConnector::get_from(&self.skills, &skill_id)
    }

    #[func]
    pub(crate) fn get_skills(&self) -> Array<Dictionary> {
        DbConnector::get_array_from(&self.skills)
    }

    #[func]
    pub(crate) fn has_skill(&self, skill_id: DbId) -> bool {
        self.skills.contains_key(&skill_id)
    }

    #[func]
    pub(crate) fn get_unit(&self, unit_id: DbId) -> Dictionary {
        DbConnector::get_from(&self.units, &unit_id)
    }

    #[func]
    pub(crate) fn get_units(&self) -> Array<Dictionary> {
        DbConnector::get_array_from(&self.units)
    }

    #[func]
    pub(crate) fn has_unit(&self, unit_id: DbId) -> bool {
        self.units.contains_key(&unit_id)
    }

    #[func]
    pub(crate) fn get_personality(&self, personality_id: DbId) -> Dictionary {
        DbConnector::get_from(&self.personalities, &personality_id)
    }

    #[func]
    pub(crate) fn has_personality(&self, personality_id: DbId) -> bool {
        self.personalities.contains_key(&personality_id)
    }

    #[func]
    fn verify_database(&self) -> bool {
        #[cfg(feature = "verify_database")]
        {
            self.verify()
        }
        #[cfg(not(feature = "verify_database"))]
        {
            // This warning is shown when the "verify_database" feature flag is not enabled.
            godot_warn!("Database verification is not enabled. Ignoring...");
            true
        }
    }
}
