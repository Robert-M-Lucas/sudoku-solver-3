use board::Possibilities;
use solution::Solution;
use crate::solver::solve_backtracking;

mod board;
mod solver;
mod solution;

fn main() {
    let sample = Solution::load("data/sudoku.txt");

    println!("{}", Possibilities::from_solution(&sample));

    let solution = solve_backtracking(sample);

    if let Some(solution) = solution {
        println!("{}", solution);
    }
    else {
        println!("No solution found");
    }
}
