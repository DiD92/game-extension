use godot::prelude::*;
use std::collections::HashSet;

pub trait ToVariantArray {
    fn to_variant_array(&self) -> VariantArray;
}

impl<T> ToVariantArray for Vec<T>
where
    T: ToGodot,
{
    fn to_variant_array(&self) -> VariantArray {
        self.iter().map(T::to_variant).collect()
    }
}

impl<T> ToVariantArray for HashSet<T>
where
    T: ToGodot,
{
    fn to_variant_array(&self) -> VariantArray {
        self.iter().map(T::to_variant).collect()
    }
}

pub trait ToVariantOption {
    fn maybe_to_variant(&self) -> Variant;
}

impl ToVariantOption for Option<GString> {
    fn maybe_to_variant(&self) -> Variant {
        self.clone().unwrap_or_default().to_variant()
    }
}

impl ToVariantOption for Option<StringName> {
    fn maybe_to_variant(&self) -> Variant {
        self.clone().unwrap_or_default().to_variant()
    }
}

impl ToVariantOption for Option<i8> {
    fn maybe_to_variant(&self) -> Variant {
        self.or(Some(-1)).unwrap_or_default().to_variant()
    }
}

pub trait ToDefaultVariant {
    fn to_default_variant(&self) -> Variant;
}

impl<'v, T> ToDefaultVariant for Option<T>
where
    T: ToGodot,
    T::ToVia<'v>: Default + ToGodot,
    T: 'v,
{
    fn to_default_variant(&self) -> Variant {
        self.as_ref()
            .map(T::to_variant)
            .unwrap_or(T::ToVia::default().to_variant())
    }
}

/*pub trait ToDictionary {
    fn to_dictionary(&self) -> Dictionary;
}

impl<K, V> ToDictionary for HashMap<K, V>
where
    K: ToGodot,
    V: ToGodot,
{
    fn to_dictionary(&self) -> Dictionary {
        self.iter()
            .map(|(k, v)| (k.to_variant(), v.to_variant()))
            .collect()
    }
}*/

pub trait GetAs {
    fn get_as<K, T>(&self, key: K, value: T) -> T
    where
        K: ToGodot,
        T: FromGodot;
}

impl GetAs for Dictionary {
    fn get_as<K, T>(&self, key: K, value: T) -> T
    where
        K: ToGodot,
        T: FromGodot,
    {
        if let Some(entry) = self.get(key) {
            if let Ok(value) = T::try_from_variant(&entry) {
                value
            } else {
                godot_warn!("GetAs failed to convert value from variant!");
                value
            }
        } else {
            value
        }
    }
}

pub trait GetVariantOr {
    fn get_or<K, T>(&self, key: K, default: T) -> Variant
    where
        K: ToGodot,
        T: ToGodot;
}

impl GetVariantOr for Dictionary {
    fn get_or<K, T>(&self, key: K, default: T) -> Variant
    where
        K: ToGodot,
        T: ToGodot,
    {
        self.get(key).unwrap_or(Variant::from(default))
    }
}

// TODO: We can remove this once Godot 4.4 lands!
pub trait FromGstringVariant {
    type Output;
    // Attempts to cast a Variant that we know is a GStrign to a StringName
    // Panics if 'variant' is not a 'GString'
    fn from_gstring_variant_to_string_name(variant: &Variant) -> Self::Output;
}

impl FromGstringVariant for StringName {
    type Output = StringName;

    fn from_gstring_variant_to_string_name(variant: &Variant) -> Self::Output {
        StringName::from(GString::from_variant(variant))
    }
}
