#![feature(drain_filter)]

mod usecases;
mod entities;

fn main() {
    let board = usecases::generate_board::execute();
    board.print();
    let new_board = usecases::remove_cells::execute(board);
    println!("");
    new_board.print();
    let solutioned = usecases::solution::execute(new_board);
    println!("");
    solutioned.print();
}
