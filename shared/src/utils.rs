use bevy::prelude::*;
use bevy_renet::renet::RenetError;

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
