use std::time::SystemTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
  pub message: String,
  pub from: String,
  pub timestamp: SystemTime,
  pub is_system: bool,
}
impl ChatMessage {
  #[inline] pub fn new(message: String, from: String) -> Self {
    ChatMessage {
      message, from,
      timestamp: SystemTime::now(),
      is_system: false
    }
  }
  #[inline] pub fn system_message(message: String) -> Self {
    ChatMessage {
      message,
      from: "[SYSTEM]".into(),
      timestamp: SystemTime::now(),
      is_system: true
    }
  }
}
