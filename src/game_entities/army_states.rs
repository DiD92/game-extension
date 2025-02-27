use crate::{database::army::ArmyId, validator::ensure_keys_are_present};

use godot::prelude::*;

enum ArmyStateFlags {
    Allied = 0b0001,
    Enemy = 0b0010,
    Active = 0b0100,
    Player = 0b1000,
}

#[derive(GodotClass, Default, Clone)]
#[class(no_init, base=RefCounted)]
pub(crate) struct ArmyStates {
    pub(crate) player_army: ArmyId,
    pub(crate) enemy_armies: Vec<ArmyId>,
    pub(crate) allied_armies: Vec<ArmyId>,
    pub(crate) active_armies: Vec<ArmyId>,
    pub(crate) participant_armies: Vec<ArmyId>,
    pub(crate) current_phase_idx: usize,
    pub(crate) current_turn: u16,
}

impl ArmyStates {
    /// Check if the internal state is actually valid.
    /// Used to decide if the instance should be passed back to Godot.
    fn is_valid(&self) -> bool {
        let mut is_valid = true;

        if self.player_army.is_empty() {
            godot_error!("'player_army' is empty!");
            is_valid = false;
        }

        if self.enemy_armies.contains(&self.player_army) {
            godot_error!("'player_army' found in 'enemy_armies'!");
            is_valid = false;
        }

        if self.allied_armies.contains(&self.player_army) {
            godot_error!("'player_army' found in 'allied_armies'!");
            is_valid = false;
        }

        for enemy_army in self.enemy_armies.iter() {
            if self.allied_armies.contains(enemy_army) {
                godot_error!("'enemy_armies' army_id found in 'allied_armies'!");
                is_valid = false;
            }
        }

        if self.active_armies.is_empty() {
            godot_error!("'active_armies' is empty!");
            is_valid = false;
        }

        if self.participant_armies.len() != (self.enemy_armies.len() + self.allied_armies.len() + 1)
        {
            godot_error!("'participant_armies' has invalid lenght!");
            is_valid = false;
        }

        if self.current_phase_idx > self.participant_armies.len() {
            godot_error!(
                "'current_phase_idx' has invalid value [{}] > [{}]!",
                self.current_phase_idx,
                self.participant_armies.len()
            );
            is_valid = false;
        }

        is_valid
    }
}

const EXPECTED_DB_PARAMS_KEYS: &[&str] = &[
    "player_army",
    "enemy_armies",
    "allied_armies",
    "active_armies",
    "starting_army",
];

const EXPECTED_STATE_PARAMS_KEYS: &[&str] = &["army_states", "current_turn", "current_phase_idx"];

#[godot_api]
impl ArmyStates {
    /// Tries to initialize the ArmyStates with data from a database chapter segment.
    ///
    /// The expected structre of the `db_segment_params` is the following:
    ///```
    /// {
    ///     player_army: <army_id>,
    ///     enemy_armies: [<army_id>, ..],
    ///     allied_armies: [<army_id>, ..],
    ///     active_armies: [<army_id>, ..],
    ///     starting_army: <army_id>,
    /// }
    ///```
    /// Will return **null** if validation failed!
    #[func]
    fn from_db_segment_params(db_segment_params: Dictionary) -> Option<Gd<Self>> {
        if !ensure_keys_are_present(&db_segment_params, EXPECTED_DB_PARAMS_KEYS) {
            return None;
        }

        let mut states = Self::default();

        states.player_army = ArmyId::from_variant(&db_segment_params.at("player_army"));

        states.enemy_armies = VariantArray::from_variant(&db_segment_params.at("enemy_armies"))
            .iter_shared()
            .map(|e| ArmyId::from_variant(&e))
            .collect();
        states.allied_armies = VariantArray::from_variant(&db_segment_params.at("allied_armies"))
            .iter_shared()
            .map(|e| ArmyId::from_variant(&e))
            .collect();
        states.active_armies = VariantArray::from_variant(&db_segment_params.at("active_armies"))
            .iter_shared()
            .map(|e| ArmyId::from_variant(&e))
            .collect();

        states.participant_armies = [states.player_army.clone()]
            .into_iter()
            .chain(states.enemy_armies.clone())
            .chain(states.allied_armies.clone())
            .collect();

        let starting_army = ArmyId::from_variant(&db_segment_params.at("starting_army"));
        states.current_phase_idx = states
            .participant_armies
            .iter()
            .position(|participant| participant == &starting_army)
            .unwrap_or_default();
        states.current_turn = 1;

        if states.is_valid() {
            Some(Gd::from_object(states))
        } else {
            None
        }
    }

    /// Tries to initialize the ArmyStates with data from a save state segment.
    ///
    /// The expected structre of the `state_segment_params` is the following:
    ///```
    /// {
    ///     army_states: [{<army_id>: <army_state>}, ..]
    ///     current_turn: <turn_number>,
    ///     current_phase_idx: <phase_idx_number>,
    /// }
    ///```
    /// Will return **null** if validation failed!
    #[func]
    fn from_state_segment_params(state_segment_params: Dictionary) -> Option<Gd<Self>> {
        if !ensure_keys_are_present(&state_segment_params, EXPECTED_STATE_PARAMS_KEYS) {
            return None;
        }

        let mut states = Self::default();

        for state_entry in VariantArray::from_variant(&state_segment_params.at("army_states"))
            .iter_shared()
            .map(|e| Dictionary::from_variant(&e))
        {
            let army_id = ArmyId::from_variant(&state_entry.at("army_id"));
            let army_state = u8::from_variant(&state_entry.at("army_state"));

            if (army_state & (ArmyStateFlags::Player as u8)) != 0 {
                states.player_army = army_id.clone();
            }

            if (army_state & (ArmyStateFlags::Enemy as u8)) != 0 {
                states.enemy_armies.push(army_id.clone());
            }

            if (army_state & (ArmyStateFlags::Allied as u8)) != 0 {
                states.allied_armies.push(army_id.clone());
            }

            if (army_state & (ArmyStateFlags::Active as u8)) != 0 {
                states.active_armies.push(army_id.clone());
            }
        }

        states.participant_armies = [states.player_army.clone()]
            .into_iter()
            .chain(states.enemy_armies.clone())
            .chain(states.allied_armies.clone())
            .collect();

        states.current_turn = u16::from_variant(&state_segment_params.at("current_turn"));
        states.current_phase_idx =
            u8::from_variant(&state_segment_params.at("current_phase_idx")) as usize;

        if states.is_valid() {
            Some(Gd::from_object(states))
        } else {
            None
        }
    }

    #[func]
    fn duplicate(&self) -> Gd<Self> {
        Gd::from_object(self.clone())
    }

    #[func]
    fn is_participant(&self, army_id: ArmyId) -> bool {
        self.participant_armies.contains(&army_id)
    }

    #[func]
    fn iter_participants(&self) -> Array<ArmyId> {
        self.participant_armies.iter().cloned().collect()
    }

    #[func]
    fn is_enemy(&self, army_id: ArmyId) -> bool {
        self.enemy_armies.contains(&army_id)
    }

    #[func]
    fn iter_enemies(&self) -> Array<ArmyId> {
        self.enemy_armies.iter().cloned().collect()
    }

    #[func]
    fn is_ally(&self, army_id: ArmyId) -> bool {
        self.allied_armies.contains(&army_id)
    }

    #[func]
    fn iter_allies(&self) -> Array<ArmyId> {
        self.allied_armies.iter().cloned().collect()
    }

    #[func]
    fn is_active(&self, army_id: ArmyId) -> bool {
        self.active_armies.contains(&army_id)
    }

    #[func]
    fn iter_active(&self) -> Array<ArmyId> {
        self.active_armies.iter().cloned().collect()
    }

    /// Tries to get the currently active army_id.
    /// Returns an empty id if not found
    #[func]
    fn try_get_active_army_id(&self) -> ArmyId {
        if let Some(army_id) = self.participant_armies.get(self.current_phase_idx) {
            army_id.clone()
        } else {
            ArmyId::default()
        }
    }

    #[func]
    fn is_player_army(&self, army_id: ArmyId) -> bool {
        self.player_army == army_id
    }

    #[func]
    fn get_player_army(&self) -> ArmyId {
        self.player_army.clone()
    }

    #[func]
    fn get_current_phase_idx(&self) -> u8 {
        self.current_phase_idx as u8
    }

    #[func]
    fn increase_phase_idx(&mut self) -> u8 {
        let participant_count = self.participant_armies.len();

        let mut next_phase_idx = (self.current_phase_idx + 1) % participant_count;

        if next_phase_idx == 0 {
            self.current_turn += 1;
        }

        while !self
            .active_armies
            .contains(&self.participant_armies[next_phase_idx])
        {
            next_phase_idx = (next_phase_idx + 1) % participant_count;
            if next_phase_idx == 0 {
                self.current_turn += 1;
            }
        }

        self.current_phase_idx = next_phase_idx;
        self.current_phase_idx as u8
    }

    #[func]
    fn get_current_turn(&self) -> u16 {
        self.current_turn
    }
}
