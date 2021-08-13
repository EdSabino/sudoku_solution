use crate::entities::Cell;
use crate::entities::Board;
use rand::{ ThreadRng, Rng };

pub fn execute(board: Board) -> Board {
  let mut solution = Solution::new(board);
  solution.execute(Cell::new(0));
  solution.board
}

struct Solution {
  original: Board,
  board: Board,
  solve_board: Board,
  rng: ThreadRng
}

impl Solution {
    pub fn new(full_board: Board) -> Self {
      Solution {
        board: full_board.clone(),
        original: full_board,
        solve_board: Board::new(),
        rng: rand::thread_rng()
      }
    }

    fn execute(&mut self, start_cell: Cell) -> bool {
      let mut cell = start_cell;
      while cell.index < 81 && self.board.board[cell.index] != 0 {
        cell = cell.next();
      }
      if cell.index == 81 {
        return true;
      }

      let mut choices = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
      for _ in 0..9 {
        match self.rng.choose(&choices) {
          Some(num_int) => {
            let num = *num_int;
            choices.retain(|&x| x != num);
            if self.board.check_valid_number_for_cell(&cell, num) {
                
              self.board.add_num(&cell, num);
              
              if self.execute(cell.next()) {
                  return true;
              } else {
                  self.board.remove_num(&cell, num);
              }
            }
          },
          None => break
        }
      }
      false
    }
}
