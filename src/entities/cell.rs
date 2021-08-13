use std::convert::TryInto;
use super::completable::Completable;

pub struct Cell {
    pub index: usize,
    pub row: usize,
    pub col: usize,
    pub bx: usize
}

impl Cell {
    pub fn new(index: usize) -> Self {
        let col = index%9;
        let row = index/9;
        Cell {
            index: index,
            row: row.try_into().unwrap(),
            col: col.try_into().unwrap(),
            bx: (((row/3)*3) + (col/3)).try_into().unwrap()
        }
    }

    pub fn next(&self) -> Self {
        Cell::new(self.index + 1)
    }

    pub fn check_row(&self, rows: &Vec<Completable>, num: i32) -> bool {
        rows[self.row].check(num.try_into().unwrap())
    }

    pub fn check_col(&self, cols: &Vec<Completable>, num: i32) -> bool {
        cols[self.col].check(num.try_into().unwrap())
    }

    pub fn check_box(&self, boxes: &Vec<Completable>, num: i32) -> bool {
        boxes[self.bx].check(num.try_into().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_have_right_coordinates_0() {
        let cell = Cell::new(0);
        assert_eq!(cell.row, 0);
        assert_eq!(cell.col, 0);
        assert_eq!(cell.bx, 0);
        assert_eq!(cell.index, 0);
    }

    #[test]
    fn must_have_right_coordinates_34() {
        let cell = Cell::new(34);
        assert_eq!(cell.row, 3);
        assert_eq!(cell.col, 7);
        assert_eq!(cell.bx, 5);
        assert_eq!(cell.index, 34);
    }

    #[test]
    fn must_go_to_next_cell() {
        let cell = Cell::new(11);
        assert_eq!(cell.row, 1);
        assert_eq!(cell.col, 2);
        assert_eq!(cell.bx, 0);
        assert_eq!(cell.index, 11);
        let next_cell = cell.next();
        assert_eq!(next_cell.row, 1);
        assert_eq!(next_cell.col, 3);
        assert_eq!(next_cell.bx, 1);
        assert_eq!(next_cell.index, 12);
    }

    #[test]
    fn check_row_must_return_false() {
        let cell = Cell::new(11);
        let mut rows = vec![Completable::new(); 9];
        rows[1].add_num(4);
        assert!(!cell.check_row(&rows, 4));
    }

    #[test]
    fn check_row_must_return_true() {
        let cell = Cell::new(11);
        let rows = vec![Completable::new(); 9];
        assert!(cell.check_row(&rows, 4));
    }

    #[test]
    fn check_col_must_return_false() {
        let cell = Cell::new(11);
        let mut cols = vec![Completable::new(); 9];
        cols[2].add_num(4);
        assert!(!cell.check_col(&cols, 4));
    }

    #[test]
    fn check_col_must_return_true() {
        let cell = Cell::new(11);
        let cols = vec![Completable::new(); 9];
        assert!(cell.check_col(&cols, 4));
    }

    #[test]
    fn check_box_must_return_false() {
        let cell = Cell::new(11);
        let mut bx = vec![Completable::new(); 9];
        bx[0].add_num(4);
        assert!(!cell.check_box(&bx, 4));
    }

    #[test]
    fn check_box_must_return_true() {
        let cell = Cell::new(11);
        let bx = vec![Completable::new(); 9];
        assert!(cell.check_box(&bx, 4));
    }
}