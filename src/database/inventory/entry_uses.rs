use godot::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, de::Visitor};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EntryUses {
    #[default]
    Infinite,
    NoUses,
    Finite(u8),
}

impl Serialize for EntryUses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            EntryUses::Infinite => serializer.serialize_i8(-1),
            EntryUses::NoUses => serializer.serialize_i8(-2),
            EntryUses::Finite(uses) => serializer.serialize_i8(*uses as i8),
        }
    }
}

struct EntryUsesVisitor;

impl Visitor<'_> for EntryUsesVisitor {
    type Value = EntryUses;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("an integer between -2 and u8::MAX")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            -1 => Ok(Self::Value::Infinite),
            -2 => Ok(Self::Value::NoUses),
            x => Ok(Self::Value::Finite(x as u8)),
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i32(value as i32)
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i32(value as i32)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i32(value as i32)
    }
}

impl<'de> Deserialize<'de> for EntryUses {
    fn deserialize<D>(deserializer: D) -> Result<EntryUses, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i8(EntryUsesVisitor)
    }
}

impl GodotConvert for EntryUses {
    type Via = i8;
}

impl ToGodot for EntryUses {
    type ToVia<'v> = i8;

    fn to_godot(&self) -> Self::Via {
        match self {
            EntryUses::Infinite => -1,
            EntryUses::NoUses => -2,
            EntryUses::Finite(value) => 0_i8.saturating_add_unsigned(*value),
        }
    }
}

impl FromGodot for EntryUses {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self::from_godot(via))
    }

    fn from_godot(via: Self::Via) -> Self {
        match via {
            -1 => EntryUses::Infinite,
            -2 => EntryUses::NoUses,
            uses if uses >= 0 => Self::Finite(via as u8),
            _ => panic!("Invalid entry uses! [{}]", via),
        }
    }
}
