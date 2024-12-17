use std::hint::black_box;
use std::time::Instant;
use board::Possibilities;
use solution::Solution;
use crate::solver::solve_backtracking;

mod board;
mod solver;
mod solution;

fn main() {
    let sample = Solution::load("data/sudoku.txt");

    // println!("{:?}", Possibilities::from_solution(&sample));

    let start = Instant::now();
    let runs = 1_000_000;
    for _ in 0..runs {
        let solution = solve_backtracking(sample.clone());
        black_box(solution);
    }
    let duration = start.elapsed();
    println!("Runs: {runs} | Duration: {:?} | Time per: {:?}", duration, duration / runs);

    // if let Some(solution) = solve_backtracking(sample) {
    //     println!("{}", solution);
    // }
    // else {
    //     println!("No solution found");
    // }
}
