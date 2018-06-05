use board::Board;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::mem;
use {Move, Pos, LEVEL_SIZE, NUM_MOVES};

pub struct Level {
  board: Board,
  start: Pos,
  goal: Pos,
  moves: u8,
  name: String,
}

impl Level {
  pub fn load(filename: &str) -> Level {
    let f = File::open(filename).expect("");
    let mut reader = BufReader::new(f);
    let mut buf = String::new();

    reader.read_line(&mut buf).expect("");
    buf.pop();
    let moves = buf.parse::<u8>().unwrap();

    let mut current_pos = Pos(0, 0);

    let mut board_str = String::new();
    reader.read_to_string(&mut board_str).expect("");
    board_str.pop();

    let mut start = current_pos;
    let mut goal = current_pos;
    let mut board = Board([false; LEVEL_SIZE]);
    for c in board_str.chars() {
      match c {
        '\n' => {
          current_pos.1 += 1;
          current_pos.0 = 0;
        }
        'X' => {
          board[current_pos] = true;
          current_pos.0 += 1;
        }
        'S' => {
          start = current_pos;
          current_pos.0 += 1;
        }
        'G' => {
          goal = current_pos;
          current_pos.0 += 1;
        }
        'O' => {
          current_pos.0 += 1;
        }
        _ => (),
      }
    }

    let name = filename.to_string();
    Level {
      board,
      start,
      goal,
      moves,
      name,
    }
  }

  pub fn solve_all() {
    let mut def_levels: Vec<_> = fs::read_dir("levels/default")
      .unwrap()
      .map(|r| r.unwrap())
      .collect();
    let mut diff_levels: Vec<_> = fs::read_dir("levels/difficult")
      .unwrap()
      .map(|r| r.unwrap())
      .collect();
    let mut lud_levels: Vec<_> = fs::read_dir("levels/ludicrous")
      .unwrap()
      .map(|r| r.unwrap())
      .collect();

    def_levels.sort_by_key(|dir| dir.path());
    diff_levels.sort_by_key(|dir| dir.path());
    lud_levels.sort_by_key(|dir| dir.path());

    let mut all_levels = def_levels;
    all_levels.append(&mut diff_levels);
    all_levels.append(&mut lud_levels);

    for level in all_levels {
      Level::load(level.path().to_str().unwrap()).solve();
    }
  }

  pub fn solve(&self) {
    if !self.solve_aux(self.moves, self.start, Vec::new()) {
      println!("No solution");
    }
  }

  fn solve_aux(&self, moves_remaining: u8, player: Pos, moves: Vec<Move>) -> bool {
    // Check if we've reached the goal
    if player == self.goal {
      println!("Solved {} in {} moves:", self.name, moves.len());
      println!("{:?}", moves);
      return true;
    }

    // Out of moves, no solution found
    if moves_remaining == 0 {
      return false;
    }

    let new_moves_remaining = moves_remaining - 1;

    // Try each of the moves and solve the resulting subproblem
    for i in 0..NUM_MOVES {
      let mv;
      unsafe {
        mv = mem::transmute::<u8, Move>(i);
      }

      let mut new_moves = moves.clone();
      new_moves.push(mv);

      let mut new_pos = player;
      // Move as far as possible before hitting a block in the board
      while !self.board[new_pos.do_move(mv)] {
        new_pos = new_pos.do_move(mv);
      }

      // Reject moves with no effect
      if player == new_pos {
        continue;
      }

      match moves.last() {
        None => (), // This must be the first move, can't prune
        Some(m) => {
          let mv2 = *m;
          // Reject moves that undo the previous one
          if (mv, mv2) == (Move::Up, Move::Down)
            || (mv2, mv) == (Move::Up, Move::Down)
            || (mv, mv2) == (Move::Left, Move::Right)
            || (mv2, mv) == (Move::Left, Move::Right)
          {
            continue;
          }
        }
      }

      if self.solve_aux(new_moves_remaining, new_pos, new_moves) {
        // Stop as soon as we find a solution
        return true;
      }
    }

    // If no move solves the problem, there is no solution
    false
  }
}
