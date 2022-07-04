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

//TODO check_username: return an error type instead of String
pub fn check_username(name: &str) -> Result<(), &'static str> {
  //Check length
  if !(MIN_NAME_LEN..=MAX_NAME_LEN).contains(&name.len()) {
    return Err(if name.len() < MIN_NAME_LEN {
      "Username is too short"
    } else {
      "Username is too long"
    });
  } 
  //Check banned regex
  for (regex, reason) in BANNED_NAMES_PARSED.iter() {
    if regex.is_match(name) {
      //TODO return a better reason
      return Err(reason);
    }
  }
  Ok(())
}

//TODO improve this
pub fn check_password(pwd: &str) -> Result<(), &'static str> {
  match (pwd.len() >= 6) && (pwd.len() <= 128) {
    true => Ok(()),
    false => Err("invalid password"),
  }
}

pub fn check_chat_message(_msg: &str) -> Result<(), &'static str> {
  todo!(); //TODO check_chat_message
}
