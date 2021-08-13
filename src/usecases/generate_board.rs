use crate::entities::Cell;
use crate::entities::Board;
use rand::{ ThreadRng, Rng };

pub fn execute() -> Board {
    let mut generate_board = GenerateBoard::new();
    generate_board.fill_board(Cell::new(0));
    generate_board.board
}

struct GenerateBoard {
    board: Board,
    rng: ThreadRng
}

impl GenerateBoard {
    pub fn new() -> Self {
        GenerateBoard {
            board: Board::new(),
            rng: rand::thread_rng()
        }
    }

    fn fill_board(&mut self, cell: Cell) -> bool {
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
                        
                        if self.fill_board(cell.next()) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::Completable;

    fn all_completable_is_true(completables: &Vec<Completable>) {
        for completable in completables {
            for i in 1..10 {
                assert!(!completable.check(i));
            }
        }
    }

    #[test]
    fn all_rows_must_be_fulled() {
        let board = execute();
        all_completable_is_true(&board.rows);
    }

    #[test]
    fn all_cols_must_be_fulled() {
        let board = execute();
        all_completable_is_true(&board.cols);
    }

    #[test]
    fn all_cols_must_be_boxes() {
        let board = execute();
        all_completable_is_true(&board.boxes);
    }

    #[test]
    fn must_have_fulled_board() {
        let board = execute();
        assert_eq!(board.board.len(), 81);
        for val in board.board {
            assert_ne!(val, 0);
        }
    }
}
