use super::{
    DbId, DbTable, IdColumn, NameDescColumns, inventory::InventoryId, kit::KitId, role::RoleId,
    skill::SkillId,
};
use crate::traits::{ToVariantArray, ToVariantOption};

use rust_extensions_macros::ToGodotDictionary;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) type UnitId = DbId;

#[derive(Serialize, Deserialize, ToGodotDictionary)]
pub(crate) struct UnitStats {
    pub(crate) level: u8,
    pub(crate) experience: u8,
    pub(crate) htp: u8,
    pub(crate) r#str: u8,
    pub(crate) mag: u8,
    pub(crate) def: u8,
    pub(crate) spt: u8,
    pub(crate) agi: u8,
    pub(crate) dex: u8,
    pub(crate) mov: u8,
}

#[cfg(feature = "verify_database")]
impl UnitStats {
    pub(super) fn validate(&self) -> bool {
        if self.level < 1 || self.level > 20 {
            godot_error!("Unit 'level' has to be between 1 and 20!");
            return false;
        }

        if self.experience >= 100 {
            godot_error!("Unit 'experience' has to be lower than 100!");
            return false;
        }

        if self.htp == 0 {
            godot_error!("Unit 'htp' cannot be zero!");
            return false;
        }

        if self.r#str == 0 {
            godot_error!("Unit 'str' cannot be zero!");
            return false;
        }

        if self.mag == 0 {
            godot_error!("Unit 'mag' cannot be zero!");
            return false;
        }

        if self.def == 0 {
            godot_error!("Unit 'def' cannot be zero!");
            return false;
        }

        if self.spt == 0 {
            godot_error!("Unit 'spt' cannot be zero!");
            return false;
        }

        if self.agi == 0 {
            godot_error!("Unit 'agi' cannot be zero!");
            return false;
        }

        if self.dex == 0 {
            godot_error!("Unit 'dex' cannot be zero!");
            return false;
        }

        if self.mov == 0 {
            godot_error!("Unit 'mov' cannot be zero!");
            return false;
        }

        true
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct UnitEntry {
    #[serde(flatten)]
    pub(crate) _i: IdColumn,
    #[serde(flatten)]
    pub(crate) _n: NameDescColumns,
    #[serde(default)]
    pub(crate) avatar_id: DbId,
    pub(crate) sprite_id: DbId,
    #[serde(flatten)]
    pub(crate) stats: UnitStats,
    #[serde(default)]
    pub(crate) role_ids: Vec<RoleId>,
    pub(crate) role_id: RoleId,
    #[serde(default)]
    pub(crate) kit_ids: Vec<KitId>,
    pub(crate) kit_id: KitId,
    #[serde(default)]
    pub(crate) skill_ids: Vec<SkillId>,
    #[serde(default)]
    pub(crate) personal_skill_id: Option<SkillId>,
    #[serde(default)]
    pub(crate) equipped_skill_ids: Vec<SkillId>,
    #[serde(default)]
    pub(crate) equipped_slot_idx: Option<i8>,
    #[serde(default)]
    pub(crate) physical_slots: Vec<InventoryId>,
    #[serde(default)]
    pub(crate) magical_slots: Vec<InventoryId>,
    #[serde(default)]
    pub(crate) support_slots: Vec<InventoryId>,
    #[serde(default)]
    pub(crate) item_slots: Vec<InventoryId>,
}

impl DbTable for UnitEntry {
    fn get_id(&self) -> DbId {
        self._i._id.clone()
    }
}

impl GodotConvert for UnitEntry {
    type Via = Dictionary;
}

impl ToGodot for UnitEntry {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut entry = Dictionary::new();

        entry.set("id", self._i._id.clone());
        entry.set("name", self._n.name.clone());
        entry.set("description", self._n.description.clone());

        entry.set("avatar_id", self.avatar_id.clone());
        entry.set("sprite_id", self.sprite_id.clone());

        entry.extend_dictionary(&self.stats.to_godot(), true);

        entry.set("role_ids", self.role_ids.to_variant_array());
        entry.set("role_id", self.role_id.clone());

        entry.set("kit_ids", self.kit_ids.to_variant_array());
        entry.set("kit_id", self.kit_id.clone());

        entry.set("skill_ids", self.skill_ids.to_variant_array());

        entry.set(
            "personal_skill_id",
            self.personal_skill_id.maybe_to_variant(),
        );
        entry.set(
            "equipped_skill_ids",
            self.equipped_skill_ids.to_variant_array(),
        );

        entry.set(
            "equipped_slot_idx",
            self.equipped_slot_idx.maybe_to_variant(),
        );
        entry.set("physical_slots", self.physical_slots.to_variant_array());
        entry.set("magical_slots", self.magical_slots.to_variant_array());
        entry.set("support_slots", self.support_slots.to_variant_array());
        entry.set("item_slots", self.item_slots.to_variant_array());

        entry
    }
}

#[cfg(feature = "verify_database")]
mod verify {
    use super::UnitEntry;
    use crate::database::{DbConnector, validation::VerifyTable};

    use godot::{classes::ResourceLoader, global::godot_error};

    const BASE_AVATAR_PATH: &str = "res://assets/art/unit_avatars/";

    impl VerifyTable for UnitEntry {
        fn validate(&self, db: &DbConnector) -> bool {
            if self._n.name.is_empty() {
                godot_error!("[{}] Unit 'name' cannot be empty!", self._i._id);
                return false;
            }

            if self._n.description.is_empty() {
                godot_error!("[{}] Unit 'description' cannot be empty!", self._i._id);
                return false;
            }
            if self._i._id.is_empty() || self._n.is_empty() {
                godot_error!("[{}] Invalid unit row in database!", self._i._id);
                return false;
            }

            // Graphic assets
            let avatar_path = format!("{}{}.png", BASE_AVATAR_PATH, self.avatar_id);
            if !ResourceLoader::singleton().exists(&avatar_path) {
                godot_error!("[{}] Unit 'avatar_id' cannot be empty!", self._i._id);
                return false;
            }

            if self.sprite_id.is_empty() {
                godot_error!("[{}] Unit 'sprite_id' cannot be empty!", self._i._id);
                return false;
            }

            // Stats
            if !self.stats.validate() {
                godot_error!("[{}] Unit 'stats' are not valid!", self._i._id);
                return false;
            }

            // Roles
            for role_id in self.role_ids.iter() {
                if !db.roles.contains_key(role_id) {
                    godot_error!(
                        "[{}] Unit role [{}] not found in database!",
                        self._i._id,
                        role_id
                    );
                    return false;
                }
            }

            if !self.role_ids.contains(&self.role_id) {
                godot_error!(
                    "[{}] Unit role [{}] not found in roles list!",
                    self._i._id,
                    self.role_id
                );
                return false;
            }

            // Kits
            for kit_id in self.kit_ids.iter() {
                if !db.kits.contains_key(kit_id) {
                    godot_error!(
                        "[{}] Unit kit [{}] not found in database!",
                        self._i._id,
                        kit_id
                    );
                    return false;
                }
            }

            if !self.kit_ids.contains(&self.kit_id) {
                godot_error!(
                    "[{}] Unit kit [{}] not found in kits list!",
                    self._i._id,
                    self.kit_id
                );
                return false;
            }

            // Skills
            for skill_id in self.skill_ids.iter() {
                if !db.skills.contains_key(skill_id) {
                    godot_error!(
                        "[{}] Unit skill [{}] not found in database!",
                        self._i._id,
                        skill_id
                    );
                    return false;
                }
            }

            if let Some(ref skill_id) = self.personal_skill_id {
                if !db.skills.contains_key(skill_id) {
                    godot_error!(
                        "[{}] Unit personal skill [{}] not found in database!",
                        self._i._id,
                        skill_id
                    );
                    return false;
                }
            }

            if self.equipped_skill_ids.len() > 4 {
                godot_error!(
                    "[{}] Unit cannot have more than 4 skills equipped!",
                    self._i._id
                );
                return false;
            }

            for skill_id in self.equipped_skill_ids.iter() {
                if !self.skill_ids.contains(skill_id) {
                    godot_error!(
                        "[{}] Unit equipped skill [{}] not in skill list!",
                        self._i._id,
                        skill_id,
                    );
                    return false;
                }
            }

            // Inventory
            let unit_kit = db.kits.get(&self.kit_id).unwrap();

            if self.physical_slots.len() > unit_kit.weapon_slots as usize {
                godot_error!(
                    "[{}] Unit has more physical slots than allowed for its kit!",
                    self._i._id
                );
                return false;
            }

            if self.magical_slots.len() > unit_kit.magic_slots as usize {
                godot_error!(
                    "[{}] Unit has more magical slots than allowed for its kit!",
                    self._i._id
                );
                return false;
            }

            if self.support_slots.len() > unit_kit.support_slots as usize {
                godot_error!(
                    "[{}] Unit has more support slots than allowed for its kit!",
                    self._i._id
                );
                return false;
            }

            if self.item_slots.len() > unit_kit.item_slots as usize {
                godot_error!(
                    "[{}] Unit has more item slots than allowed for its kit!",
                    self._i._id
                );
                return false;
            }

            for item_id in self
                .physical_slots
                .iter()
                .chain(self.magical_slots.iter())
                .chain(self.support_slots.iter())
                .chain(self.item_slots.iter())
            {
                if !db.inventory.contains_key(item_id) {
                    godot_error!(
                        "[{}] Unit inventory entry [{}] not found in database!",
                        self._i._id,
                        item_id,
                    );
                    return false;
                }
            }

            if let Some(slot_idx) = self.equipped_slot_idx {
                if slot_idx != -1 && !(0..=5).contains(&slot_idx) {
                    godot_error!(
                        "[{}] Unit equipped_slot_idx should be either -1 or a value between 0 and 5!",
                        self._i._id
                    );
                    return false;
                }

                if slot_idx == -1 {
                    return true;
                }

                let slot_idx = slot_idx as u8;

                let weapon_range = 0..unit_kit.weapon_slots;
                let magic_range =
                    unit_kit.weapon_slots..(unit_kit.magic_slots + unit_kit.weapon_slots);
                let support_range = (unit_kit.magic_slots + unit_kit.weapon_slots)
                    ..(unit_kit.magic_slots + unit_kit.weapon_slots + unit_kit.support_slots);

                if weapon_range.contains(&slot_idx) {
                    let phy_slots = self.physical_slots.len().saturating_sub(1);
                    if phy_slots < slot_idx as usize {
                        godot_error!(
                            "[{}] Unit equipped_slot_idx [{}] should not be empty!",
                            self._i._id,
                            slot_idx,
                        );
                        return false;
                    }
                } else if magic_range.contains(&slot_idx) {
                    let mag_slots =
                        (self.physical_slots.len() + self.magical_slots.len()).saturating_sub(1);
                    if mag_slots < slot_idx as usize {
                        godot_error!(
                            "[{}] Unit equipped_slot_idx [{}] should not be empty!",
                            self._i._id,
                            slot_idx,
                        );
                        return false;
                    }
                } else if support_range.contains(&slot_idx) {
                    let sup_slots = (self.physical_slots.len()
                        + self.magical_slots.len()
                        + self.support_slots.len())
                    .saturating_sub(1);
                    if sup_slots < slot_idx as usize {
                        godot_error!(
                            "[{}] Unit equipped_slot_idx [{}] should not be empty!",
                            self._i._id,
                            slot_idx,
                        );
                        return false;
                    }
                } else {
                    godot_error!(
                        "[{}] Unit equipped_slot_idx should be the index of an equipment slot!",
                        self._i._id
                    );
                    return false;
                }
            }

            true
        }
    }
}
