pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

pub const DEFAULT_CLIENT_VIEW_DIST: usize = 6;
//currently not used
pub const MAX_MP_VIEW_DIST: usize = 32;
pub const MAX_MP_REQ_DIST: usize = MAX_MP_VIEW_DIST + 2;

pub const DEFAULT_PORT: u16 = 12478;
pub const PROTOCOL_ID: u64 = 4;
pub const MAX_CLIENTS: usize = 64;

pub const CHANNEL_RELIABLE: u8 = 0;
pub const CHANNEL_UNRELIABLE: u8 = 1;
pub const CHANNEL_BLOCK: u8 = 2;

pub const MIN_NAME_LEN: usize = 3;
pub const MAX_NAME_LEN: usize = 24;
pub const BANNED_NAMES: &[&str] = &[
  r"\[.*\]",
  r"system", 
  r"server", 
];

//STATIC 
use once_cell::sync::Lazy;
pub static BANNED_NAMES_PARSED: Lazy<Vec<regex::Regex>> = Lazy::new(|| {
  let mut regexes = Vec::new();
  for name in BANNED_NAMES {
    regexes.push(regex::Regex::new(*name).unwrap());
  }
  regexes
});
