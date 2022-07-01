use bevy::prelude::*;
use bevy_renet::renet::RenetError;
use crate::consts::{BANNED_NAMES_PARSED, MIN_NAME_LEN, MAX_NAME_LEN};

pub fn panic_on_renet_error_system(mut renet_error: EventReader<RenetError>) {
  for error in renet_error.iter() {
    panic!("{}", error);
  }
}

pub fn print_on_renet_error_system(mut renet_error: EventReader<RenetError>) {
  for error in renet_error.iter() {
    error!("{}", error);
  }
}

//TODO check_username: return Result instead of bool
pub fn check_username(name: &str) -> bool {
  //Check length
  if !(MIN_NAME_LEN..=MAX_NAME_LEN).contains(&name.len()) {
    return false;
  } 
  //Check banned regex
  for regex in BANNED_NAMES_PARSED.iter() {
    if regex.is_match(name) {
      return false;
    }
  }
  true
}
