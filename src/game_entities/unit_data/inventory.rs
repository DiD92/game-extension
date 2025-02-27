use crate::traits::ToDefaultVariant;

use super::*;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct SlotEntry {
    pub(super) id: InventoryId,
    pub(super) idx: InventoryIdx,
    pub(super) uses: EntryUses,
}

impl GodotConvert for SlotEntry {
    type Via = VariantArray;
}

impl ToGodot for SlotEntry {
    type ToVia<'v> = VariantArray;

    fn to_godot(&self) -> Self::Via {
        varray![self.id, self.idx, self.uses]
    }
}

impl FromGodot for SlotEntry {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        Self {
            id: InventoryId::from_variant(&via.at(0)),
            idx: InventoryIdx::from_variant(&via.at(1)),
            uses: EntryUses::from_variant(&via.at(2)),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
pub(crate) struct InventorySlot {
    pub(crate) slot_type: SlotType,
    pub(crate) slot_entry: Option<SlotEntry>,
}

impl InventorySlot {
    pub(crate) fn is_empty(&self) -> bool {
        self.slot_entry.is_none()
    }

    pub(crate) fn contains_equipment(&self) -> bool {
        matches!(
            self.slot_type,
            SlotType::Physical | SlotType::Magical | SlotType::Support
        )
    }

    pub(crate) fn contains_weapon(&self) -> bool {
        matches!(self.slot_type, SlotType::Physical | SlotType::Magical)
    }

    pub(crate) fn contains_support(&self) -> bool {
        matches!(self.slot_type, SlotType::Support)
    }

    pub(crate) fn contains_item(&self) -> bool {
        matches!(self.slot_type, SlotType::Item)
    }

    pub(crate) fn get_entry(&self) -> Option<&SlotEntry> {
        self.slot_entry.as_ref()
    }

    pub(crate) fn clear_entry(&mut self) -> Option<InventoryIdx> {
        if let Some(entry) = self.slot_entry.take() {
            Some(entry.idx)
        } else {
            None
        }
    }
}

impl GodotConvert for InventorySlot {
    type Via = Dictionary;
}

impl ToGodot for InventorySlot {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::Via {
        dict! {
            "type": self.slot_type as u8,
            "entry": self.slot_entry.to_default_variant(),
        }
    }
}

impl FromGodot for InventorySlot {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        let slot_entry = via
            .get("entry")
            .map(|e| VariantArray::from_variant(&e))
            .filter(|arr| !arr.is_empty())
            .map(|e| Some(SlotEntry::from_variant(&e.to_variant())))
            .unwrap_or_default();

        let slot_type = match u8::from_variant(&via.at("type")) {
            0 => SlotType::Physical,
            1 => SlotType::Magical,
            2 => SlotType::Support,
            3 => SlotType::Item,
            _ => {
                panic!("Tried to parse InventorySlot with invalid type: {}", via);
            }
        };

        Self {
            slot_type,
            slot_entry,
        }
    }
}
