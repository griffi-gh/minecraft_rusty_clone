use bevy::prelude::*;
use bevy_renet::renet::RenetError;
use regex::Regex;
use crate::consts::BANNED_NAMES;

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

pub fn check_username(name: &str) -> bool {
  for regex in BANNED_NAMES {
    if Regex::new(*regex).unwrap().is_match(name) {
      return false;
    }
  }
  true
}
