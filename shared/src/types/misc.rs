#[repr(usize)]
#[derive(Clone, Copy)]
pub enum CubeFace {
  Top    = 0,
  Front  = 1,
  Left   = 2,
  Right  = 3,
  Back   = 4,
  Bottom = 5,
}
