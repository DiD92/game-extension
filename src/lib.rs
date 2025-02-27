use godot::prelude::*;

pub(crate) mod database;
mod game_entities;
mod pathfinding;
mod save;
pub(crate) mod traits;
pub(crate) mod validator;

#[cfg(not(any(feature = "save_json", feature = "save_bin")))]
compile_error!("Either of features 'save_json' or 'save_bin' must be enabled!");

struct RustExtensions;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtensions {}
