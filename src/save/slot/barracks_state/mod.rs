use crate::{
    database::{kit::KitId, role::RoleId, skill::SkillId, unit::UnitId},
    game_entities::unit_data::InventoryIdx,
    traits::GetVariantOr,
};

use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod inventory;
mod roles;
mod units;

pub(super) type UnitKits = HashMap<UnitId, Vec<KitId>>;
pub(super) type UnitSkills = HashMap<UnitId, Vec<SkillId>>;
pub(super) type RestingUnits = HashMap<UnitId, u8>;

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct BarracksState {
    item_idx_gen: u16,
    unit_idx_gen: u16,
    gold_amt: u32,
    inventory: inventory::InventoryEntries,
    roles: roles::UnitsRoles,
    kits: UnitKits,
    skills: UnitSkills,
    units: units::BarracksUnits,
    resting_units: RestingUnits,
}

impl BarracksState {
    pub(super) fn update_data(&mut self, update_data: Dictionary) {
        // TODO: Replace GString conversions once Godot 4.4 is released
        if let Some(data) = update_data.get("item_idx_gen") {
            self.item_idx_gen = u16::from_variant(&data);
        }
        if let Some(data) = update_data.get("unit_idx_gen") {
            self.unit_idx_gen = u16::from_variant(&data);
        }

        if let Some(data) = update_data.get("gold_amt") {
            self.gold_amt = u32::from_variant(&data);
        }

        if let Some(data) = update_data.get("inventory") {
            self.inventory = Dictionary::from_variant(&data)
                .iter_shared()
                .map(|(idx, entry)| {
                    (
                        InventoryIdx::from_variant(&idx),
                        inventory::InventoryEntry::from_variant(&entry),
                    )
                })
                .collect();
        }

        if let Some(data) = update_data.get("roles") {
            self.roles = Dictionary::from_variant(&data)
                .iter_shared()
                .map(|(unit_id, unit_roles)| {
                    (
                        UnitId::from(GString::from_variant(&unit_id)),
                        Dictionary::from_variant(&unit_roles)
                            .iter_shared()
                            .map(|(role_id, entry)| {
                                (
                                    RoleId::from(GString::from_variant(&role_id)),
                                    roles::RoleEntry::from_variant(&entry),
                                )
                            })
                            .collect(),
                    )
                })
                .collect();
        }

        if let Some(data) = update_data.get("kits") {
            self.kits = Dictionary::from_variant(&data)
                .iter_shared()
                .map(|(unit_id, unit_kits)| {
                    (
                        UnitId::from(GString::from_variant(&unit_id)),
                        VariantArray::from_variant(&unit_kits)
                            .iter_shared()
                            .map(|kit_id| KitId::from_variant(&kit_id))
                            .collect(),
                    )
                })
                .collect();
        }

        if let Some(data) = update_data.get("skills") {
            self.skills = Dictionary::from_variant(&data)
                .iter_shared()
                .map(|(unit_id, unit_skills)| {
                    (
                        UnitId::from(GString::from_variant(&unit_id)),
                        VariantArray::from_variant(&unit_skills)
                            .iter_shared()
                            .map(|skill_id| SkillId::from(GString::from_variant(&skill_id)))
                            .collect(),
                    )
                })
                .collect();
        }

        if let Some(data) = update_data.get("units") {
            self.units = Dictionary::from_variant(&data)
                .iter_shared()
                .map(|(unit_id, unit_data)| {
                    (
                        UnitId::from(GString::from_variant(&unit_id)),
                        units::UnitSaveData::from_variant(&unit_data),
                    )
                })
                .collect();
        }

        if let Some(data) = update_data.get("resting_units") {
            self.resting_units = Dictionary::from_variant(&data)
                .iter_shared()
                .map(|(unit_id, resting_chapters)| {
                    (
                        UnitId::from(GString::from_variant(&unit_id)),
                        u8::from_variant(&resting_chapters),
                    )
                })
                .collect();
        }
    }
}

impl GodotConvert for BarracksState {
    type Via = Dictionary;
}

impl ToGodot for BarracksState {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut state_dict = Dictionary::new();

        state_dict.set("item_idx_gen", self.item_idx_gen);
        state_dict.set("unit_idx_gen", self.unit_idx_gen);
        state_dict.set("gold_amt", self.gold_amt);
        state_dict.set("inventory", {
            self.inventory
                .iter()
                .map(|(entry_idx, entry)| (entry_idx.to_variant(), entry.to_variant()))
                .collect::<Dictionary>()
        });
        state_dict.set("roles", {
            self.roles
                .iter()
                .map(|(unit_id, role_entries)| {
                    (
                        unit_id.to_variant(),
                        role_entries
                            .iter()
                            .map(|(role_id, role_entry)| {
                                (role_id.to_variant(), role_entry.to_variant())
                            })
                            .collect::<Dictionary>(),
                    )
                })
                .collect::<Dictionary>()
        });
        state_dict.set("kits", {
            self.kits
                .iter()
                .map(|(unit_id, kits)| {
                    (
                        unit_id.to_variant(),
                        kits.iter()
                            .map(|kit_id| kit_id.to_variant())
                            .collect::<VariantArray>(),
                    )
                })
                .collect::<Dictionary>()
        });
        state_dict.set("skills", {
            self.skills
                .iter()
                .map(|(unit_id, skills)| {
                    (
                        unit_id.to_variant(),
                        skills
                            .iter()
                            .map(|skill_id| skill_id.to_variant())
                            .collect::<VariantArray>(),
                    )
                })
                .collect::<Dictionary>()
        });
        state_dict.set("units", {
            self.units
                .iter()
                .map(|(unit_id, unit_data)| (unit_id.to_variant(), unit_data.to_variant()))
                .collect::<Dictionary>()
        });
        state_dict.set("resting_units", {
            self.resting_units
                .iter()
                .map(|(unit_id, resting_chapters)| {
                    (unit_id.to_variant(), resting_chapters.to_variant())
                })
                .collect::<Dictionary>()
        });

        state_dict
    }
}

impl FromGodot for BarracksState {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            item_idx_gen: u16::from_variant(&via.get_or("item_idx_gen", u16::MIN)),
            unit_idx_gen: u16::from_variant(&via.get_or("unit_idx_gen", u16::MIN)),
            gold_amt: u32::from_variant(&via.get_or("gold_amt", u32::MIN)),
            inventory: Dictionary::from_variant(&via.get_or("inventory", Dictionary::default()))
                .iter_shared()
                .map(|(idx, entry)| {
                    (
                        InventoryIdx::from_variant(&idx),
                        inventory::InventoryEntry::from_variant(&entry),
                    )
                })
                .collect(),
            roles: Dictionary::from_variant(&via.get_or("roles", Dictionary::default()))
                .iter_shared()
                .map(|(unit_id, unit_roles)| {
                    (
                        UnitId::from_variant(&unit_id),
                        Dictionary::from_variant(&unit_roles)
                            .iter_shared()
                            .map(|(role_id, entry)| {
                                (
                                    RoleId::from_variant(&role_id),
                                    roles::RoleEntry::from_variant(&entry),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
            kits: Dictionary::from_variant(&via.get_or("kits", Dictionary::default()))
                .iter_shared()
                .map(|(unit_id, unit_kits)| {
                    (
                        UnitId::from_variant(&unit_id),
                        VariantArray::from_variant(&unit_kits)
                            .iter_shared()
                            .map(|kit_id| KitId::from_variant(&kit_id))
                            .collect(),
                    )
                })
                .collect(),
            skills: Dictionary::from_variant(&via.get_or("skills", Dictionary::default()))
                .iter_shared()
                .map(|(unit_id, unit_skills)| {
                    (
                        UnitId::from_variant(&unit_id),
                        VariantArray::from_variant(&unit_skills)
                            .iter_shared()
                            .map(|skill_id| SkillId::from_variant(&skill_id))
                            .collect(),
                    )
                })
                .collect(),
            units: Dictionary::from_variant(&via.get_or("units", Dictionary::default()))
                .iter_shared()
                .map(|(unit_id, unit_data)| {
                    (
                        UnitId::from_variant(&unit_id),
                        units::UnitSaveData::from_variant(&unit_data),
                    )
                })
                .collect(),
            resting_units: Dictionary::from_variant(
                &via.get_or("resting_units", Dictionary::default()),
            )
            .iter_shared()
            .map(|(unit_id, resting_chapters)| {
                (
                    UnitId::from_variant(&unit_id),
                    u8::from_variant(&resting_chapters),
                )
            })
            .collect(),
        }
    }
}
