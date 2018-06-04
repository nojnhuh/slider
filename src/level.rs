use board::Board;
use std::mem;
use {Move, Pos, LEVEL_HEIGHT, LEVEL_SIZE, LEVEL_WIDTH, NUM_MOVES};

pub struct Level {
  board: Board,
  start: Pos,
  goal: Pos,
  moves: u8,
}

impl Level {
  pub fn new(blocks: Vec<Pos>, start: Pos, goal: Pos, moves: u8) -> Level {
    let mut board = Board([false; LEVEL_SIZE]);

    // Set outer boundary
    for i in 0..LEVEL_WIDTH {
      board[Pos(i, 0)] = true;
      board[Pos(i, LEVEL_HEIGHT - 1)] = true;
    }
    for j in 1..LEVEL_HEIGHT - 1 {
      board[Pos(0, j)] = true;
      board[Pos(LEVEL_WIDTH - 1, j)] = true;
    }

    // Set level-dependent blocks
    for block in blocks {
      board[block] = true;
    }

    Level {
      board,
      start,
      goal,
      moves,
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
      println!("Solved in {} moves:", moves.len());
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
