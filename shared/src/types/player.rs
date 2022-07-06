use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Clone)]
pub struct Username(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerInitData {
  pub position: Vec3,
  pub username: String,
}
