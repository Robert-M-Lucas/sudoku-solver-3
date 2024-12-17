use board::Possibilities;
use solution::Solution;

mod board;
mod solver;
mod solution;

fn main() {
    let mut sample = Solution::load("data/sudoku.txt");


    println!("{:?}", sample);
}
