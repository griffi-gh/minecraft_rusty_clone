pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

pub const DEFAULT_CLIENT_VIEW_DIST: usize = 6;
//currently not used
pub const MAX_MP_VIEW_DIST: usize = 32;
pub const MAX_MP_REQ_DIST: usize = MAX_MP_VIEW_DIST + 2;

pub const DEFAULT_PORT: u16 = 12478;
pub const PROTOCOL_ID: u64 = 5;
pub const MAX_CLIENTS: usize = 64;

pub const CHANNEL_RELIABLE: u8 = 0;
pub const CHANNEL_UNRELIABLE: u8 = 1;
pub const CHANNEL_BLOCK: u8 = 2;

pub const MIN_NAME_LEN: usize = 3;
pub const MAX_NAME_LEN: usize = 24;
pub const BANNED_NAMES: &[(&str, &str)] = &[
  (r"[^\x20-\x7E]", "Username contains Non-ASCII characters"),
  (r"^\s*$", "Empty username"),
  (r"(^\s+)|(\s+$)", "Usernames can't start/end with spaces"),
  (r"\[.*\]", "Usernames wrapped in square brackets are not allowed"),
];

//STATIC 
mod static_stuff {
  use super::*;
  use once_cell::sync::Lazy;
  use regex::Regex;
  use bevy_renet::renet::{
    RenetConnectionConfig, ChannelConfig,
    ReliableChannelConfig, BlockChannelConfig,
    UnreliableChannelConfig, 
  };

  pub static BANNED_NAMES_PARSED: Lazy<Vec<(Regex, &'static str)>> = Lazy::new(|| {
    let mut regexes = Vec::new();
    for entry in BANNED_NAMES {
      regexes.push((
        Regex::new(entry.0).unwrap(),
        entry.1
      ));
    }
    regexes
  });

  #[inline(always)]
  pub fn renet_connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
      max_packet_size: 128 * 1024,
      channels_config: vec![
        ChannelConfig::Reliable(ReliableChannelConfig {
          packet_budget: u64::MAX,
          ..Default::default()
        }),
        ChannelConfig::Unreliable(UnreliableChannelConfig {
          max_message_size: u64::MAX,
          packet_budget: u64::MAX,
          ..Default::default()
        }),
        ChannelConfig::Block(BlockChannelConfig::default()),
      ],
      ..Default::default()
    }
  }
}
pub use static_stuff::*;
