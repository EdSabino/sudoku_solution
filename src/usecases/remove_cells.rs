use std::convert::TryInto;
use crate::entities::Board;
use crate::entities::Cell;
use rand::{ ThreadRng, Rng };

pub fn execute(board: Board) -> Board {
    let mut remove_cells = RemoveCells::new(board);
    remove_cells.execute();
    remove_cells.board
}

struct RemoveCells {
    original: Board,
    board: Board,
    solve_board: Board,
    rng: ThreadRng
}

impl RemoveCells {
    fn new(full_board: Board) -> Self {
        RemoveCells {
            board: full_board.clone(),
            original: full_board,
            solve_board: Board::new(),
            rng: rand::thread_rng()
        }
    }

    fn execute(&mut self) {
        let mut attempts = 0;
        let mut choices = (0..81).collect::<Vec<i32>>();
        while attempts < 50 {
            match self.rng.choose(&choices) {
                Some(num_int) => {
                    let num = *num_int;
                    let value = self.board.board[Cell::new(num.try_into().unwrap()).index];
                    choices = self.remove_num(choices, num);
                    self.solve_board = self.board.clone();
                    if !self.solve(Cell::new(0)) || !self.solve_board.eq(&self.original) {
                        choices = self.add_num(choices, num, value);
                        attempts += 1;
                    }
                },
                None => break
            }
        }
    }

    fn add_num(&mut self, mut choices: Vec<i32>, num: i32, value: i32) -> Vec<i32> {
        let cell = Cell::new(num.try_into().unwrap());
        self.board.add_num(&cell, value);
        choices.push(num);
        choices
    }

    fn remove_num(&mut self, mut choices: Vec<i32>, num: i32) -> Vec<i32> {
        let cell = Cell::new(num.try_into().unwrap());
        self.board.remove_num(&cell, self.board.board[cell.index]);
        choices.retain(|&x| x != num);
        choices
    }

    fn solve(&mut self, mut cell: Cell) -> bool {
        if cell.index == 81 {
            return true;
        }

        while self.solve_board.board[cell.index] != 0 {
            cell = cell.next();
            if cell.index == 81 {
                return true;
            }
        }

        let mut choices = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for _ in 0..9 {
            match self.rng.choose(&choices) {
                Some(num_int) => {
                    let num = *num_int;
                    choices.retain(|&x| x != num);
                    if self.solve_board.check_valid_number_for_cell(&cell, num) {
                        self.solve_board.add_num(&cell, num);
                        
                        if self.solve(cell.next()) {
                            return true;
                        } else {
                            self.solve_board.remove_num(&cell, num);
                        }
                    }
                },
                None => break
            }
        }
        false
    }
}