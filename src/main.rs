extern crate slider;

use slider::{Level, Pos};

fn main() {
  let _default: Vec<Level> = vec![];
  let _difficult: Vec<Level> = vec![];
  let ludicrous: Vec<Level> = vec![
    // Ludicrous 1
    Level::new(
      vec![
        Pos(8, 1),
        Pos(7, 2),
        Pos(5, 3),
        Pos(1, 4),
        Pos(4, 5),
        Pos(3, 6),
        Pos(6, 6),
        Pos(8, 9),
        Pos(5, 10),
        Pos(2, 11),
        Pos(6, 15),
        Pos(7, 16),
      ],
      Pos(3, 8),
      Pos(6, 14),
      20,
    ),
    // Ludicrous 2
    Level::new(
      vec![
        Pos(1, 1),
        Pos(6, 1),
        Pos(5, 2),
        Pos(8, 3),
        Pos(7, 4),
        Pos(5, 6),
        Pos(8, 7),
        Pos(6, 10),
        Pos(4, 11),
        Pos(8, 11),
        Pos(1, 13),
        Pos(5, 13),
        Pos(6, 15),
      ],
      Pos(5, 15),
      Pos(1, 3),
      20,
    ),
    // Ludicrous 9
    Level::new(
      vec![
        Pos(3, 1),
        Pos(6, 2),
        Pos(7, 3),
        Pos(8, 3),
        Pos(1, 4),
        Pos(8, 5),
        Pos(1, 8),
        Pos(5, 8),
        Pos(7, 8),
        Pos(2, 11),
        Pos(6, 12),
        Pos(4, 13),
        Pos(3, 14),
        Pos(5, 15),
      ],
      Pos(7, 9),
      Pos(8, 14),
      21,
    ),
    //Ludicrous 10
    Level::new(
      vec![
        Pos(2, 2),
        Pos(8, 2),
        Pos(4, 4),
        Pos(8, 5),
        Pos(1, 8),
        Pos(7, 9),
        Pos(2, 10),
        Pos(6, 10),
        Pos(5, 11),
        Pos(7, 13),
        Pos(4, 14),
        Pos(6, 16),
        Pos(1, 16),
      ],
      Pos(6, 9),
      Pos(3, 16),
      22,
    ),
    //Ludicrous 11
    Level::new(
      vec![
        Pos(2, 2),
        Pos(5, 2),
        Pos(1, 3),
        Pos(8, 3),
        Pos(5, 6),
        Pos(1, 7),
        Pos(4, 7),
        Pos(2, 8),
        Pos(4, 9),
        Pos(6, 9),
        Pos(1, 10),
        Pos(6, 11),
        Pos(3, 12),
        Pos(7, 13),
        Pos(1, 14),
        Pos(3, 14),
        Pos(4, 16),
      ],
      Pos(5, 7),
      Pos(3, 15),
      22,
    ),
  ];
  for l in &ludicrous {
    l.solve();
  }
  // ludicrous[2].solve();
}
