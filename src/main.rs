use board::Board;

mod board;


fn main() {
    let mut sample = Board::load("data/sudoku.txt");


    println!("{:?}", sample);
}
