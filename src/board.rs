use std::ops::{Index, IndexMut};
use {Pos, LEVEL_HEIGHT, LEVEL_SIZE};

pub struct Board(pub [bool; LEVEL_SIZE]);

impl Board {
  fn raw_index(&self, pos: Pos) -> usize {
    pos.0 * LEVEL_HEIGHT + pos.1
  }
}

impl Index<Pos> for Board {
  type Output = bool;

  fn index(&self, pos: Pos) -> &bool {
    &self.0[self.raw_index(pos)]
  }
}

impl IndexMut<Pos> for Board {
  fn index_mut(&mut self, pos: Pos) -> &mut bool {
    &mut self.0[self.raw_index(pos)]
  }
}
