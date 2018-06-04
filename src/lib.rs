mod board;
pub mod level;

pub const LEVEL_WIDTH: usize = 10;
pub const LEVEL_HEIGHT: usize = 18;
pub const LEVEL_SIZE: usize = LEVEL_WIDTH * LEVEL_HEIGHT;

const NUM_MOVES: u8 = 4;

pub use level::Level;

#[derive(Copy, Clone, PartialEq)]
pub struct Pos(pub usize, pub usize);

impl Pos {
  fn do_move(&self, mv: Move) -> Pos {
    match mv {
      Move::Up => Pos(self.0, self.1 + 1),
      Move::Down => Pos(self.0, self.1 - 1),
      Move::Left => Pos(self.0 - 1, self.1),
      Move::Right => Pos(self.0 + 1, self.1),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
enum Move {
  Up,
  Down,
  Left,
  Right,
}
