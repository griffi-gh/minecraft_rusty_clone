pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

pub const DEFAULT_CLIENT_VIEW_DIST: usize = 4;
//currently not used
pub const MAX_MP_VIEW_DIST: usize = 32;
pub const MAX_MP_REQ_DIST: usize = MAX_MP_VIEW_DIST + 2;

pub const DEFAULT_PORT: u16 = 12478;
pub const PROTOCOL_ID: u64 = 1;
pub const MAX_CLIENTS: usize = 64;

pub const CHANNEL_RELIABLE: u8 = 0;
pub const CHANNEL_UNRELIABLE: u8 = 1;
pub const CHANNEL_BLOCK: u8 = 2;

//Compatability
#[deprecated = "Use PROTOCOL_ID instead"]
pub const PROTOCOL_VERSION: u16 = PROTOCOL_ID as u16;
#[deprecated = "Use DEFAULT_PORT instead"]
pub const PORT: u16 = DEFAULT_PORT;

//pub const BANNED_NAMES: [&str; 4] = ["[system]", "[server]", "system", "server",];
