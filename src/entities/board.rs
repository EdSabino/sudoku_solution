use std::convert::TryInto;
use super::completable::Completable;
use super::cell::Cell;

#[derive(Clone)]
pub struct Board {
    pub rows: Vec<Completable>,
    pub cols: Vec<Completable>,
    pub boxes: Vec<Completable>,
    pub board: Vec<i32>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            rows: vec![Completable::new(); 9],
            cols: vec![Completable::new(); 9],
            boxes: vec![Completable::new(); 9],
            board: vec![0; 81],
        }
    }

    pub fn print(&self) {
        let mut i = 1;
        for num in self.board.iter() {
            if *num == 0 {
                print!("x ")
            } else {
                print!("{} ", num);
            }
            if i % 9 == 0 {
                println!("");
            }
            i += 1;
        }
    }

    pub fn add_num(&mut self, cell: &Cell, num: i32) {
        let usize_num = num.try_into().unwrap();
        self.rows[cell.row].add_num(usize_num);
        self.cols[cell.col].add_num(usize_num);
        self.boxes[cell.bx].add_num(usize_num);
        self.board[cell.index] = num;
    }

    pub fn remove_num(&mut self, cell: &Cell, num: i32) {
        let usize_num = num.try_into().unwrap();
        self.rows[cell.row].remove_num(usize_num);
        self.cols[cell.col].remove_num(usize_num);
        self.boxes[cell.bx].remove_num(usize_num);
        self.board[cell.index] = 0;
    }

    pub fn check_valid_number_for_cell(&self, cell: &Cell, num: i32) -> bool {
        cell.check_box(&self.boxes, num) && cell.check_col(&self.cols, num) && cell.check_row(&self.rows, num)
    }

    pub fn eq(&self, board: &Board) -> bool {
        self.board.iter().zip(board.board.iter()).filter(|&(a, b)| a == b).count() == 81
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_create_board_9x9() {
        let board = Board::new();
        assert_eq!(board.rows.len(), 9);
        assert_eq!(board.cols.len(), 9);
        assert_eq!(board.boxes.len(), 9);
        assert_eq!(board.board.len(), 81);
    }

    #[test]
    fn must_add_num_to_all() {
        let mut board = Board::new();
        let cell = Cell::new(11);
        board.add_num(&cell, 4);

        assert!(!board.rows[cell.row].check(4));
        assert!(!board.cols[cell.col].check(4));
        assert!(!board.boxes[cell.bx].check(4));
        assert_eq!(board.board[cell.index], 4);
    }

    #[test]
    fn must_remove_num_from_all() {
        let mut board = Board::new();
        let cell = Cell::new(11);
        board.add_num(&cell, 4);
        board.remove_num(&cell, 4);

        assert!(board.rows[cell.row].check(4));
        assert!(board.cols[cell.col].check(4));
        assert!(board.boxes[cell.bx].check(4));
        assert_eq!(board.board[cell.index], 0);
    }

    #[test]
    fn must_be_valid_num_to_cell() {
        let mut board = Board::new();
        let cell = Cell::new(11);
        board.add_num(&cell, 4);
        let next_cell = cell.next();

        assert!(board.check_valid_number_for_cell(&next_cell, 5));
    }

    #[test]
    fn must_not_be_valid_num_to_cell_on_row() {
        let mut board = Board::new();
        let cell = Cell::new(11);
        board.add_num(&cell, 4);
        let next_cell = cell.next();

        assert!(!board.check_valid_number_for_cell(&next_cell, 4));
    }

    #[test]
    fn must_not_be_valid_num_to_cell_on_col() {
        let mut board = Board::new();
        let cell = Cell::new(11);
        board.add_num(&cell, 4);
        let next_cell = Cell::new(2);

        assert!(!board.check_valid_number_for_cell(&next_cell, 4));
    }

    #[test]
    fn must_not_be_valid_num_to_cell_on_box() {
        let mut board = Board::new();
        let cell = Cell::new(11);
        board.add_num(&cell, 4);
        let next_cell = Cell::new(1);

        assert!(!board.check_valid_number_for_cell(&next_cell, 4));
    }
}
