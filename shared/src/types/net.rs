use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthUserData { 
  pub username: String
}

#[derive(Debug, Default)]
pub struct Lobby {
  pub players: bevy::utils::HashMap<u64, bevy::prelude::Entity>,
}
