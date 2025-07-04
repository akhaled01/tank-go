use bevy::prelude::*;
use shared::{GameState, Player};
use std::collections::HashMap;

#[derive(Component)]
pub struct RemotePlayer {
    pub id: String,
}

#[derive(Resource, Default)]
pub struct GameData {
    pub my_id: Option<String>,
    pub players: HashMap<String, Player>,
    pub state: Option<GameState>,
    pub game_start_time: Option<f64>,
    pub player_entities: HashMap<String, Entity>,
}

// Component to mark the local player
#[derive(Component)]
pub struct LocalPlayer;
