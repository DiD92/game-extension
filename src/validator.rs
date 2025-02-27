use godot::prelude::*;

/// Ensures the given `dict` contains all entries defined in `keys`.
/// Panics if any of the expected keys is missing from the dictionary.
pub fn ensure_keys_are_present(dict: &Dictionary, keys: &[&str]) -> bool {
    if !keys.iter().all(|k| dict.contains_key(*k)) {
        godot_error!(
            "Dictionary is missing one or more expected keys!: {:?}",
            keys
        );
        false
    } else {
        true
    }
}
