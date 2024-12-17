use board::Possibilities;
use solution::Solution;
use crate::solver::solve_backtracking;

mod board;
mod solver;
mod solution;

fn main() {
    let sample = Solution::load("data/sudoku.txt");

    let solution = solve_backtracking(sample);

    println!("{}", solution.unwrap());
}
