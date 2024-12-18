use crate::solver::solve_backtracking;
use solution::Solution;
use std::hint::black_box;
use std::time::Instant;
use thousands::Separable;

mod board;
mod solution;
mod solver;

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

    if let Some(solution) = solve_backtracking(sample) {
        println!("{}", solution);
    } else {
        println!("No solution found");
    }

    println!(
        "Runs: {} | Duration: {:?} | Time per: {:?} | Per second: {}",
        runs.separate_with_commas(),
        duration,
        duration / runs,
        (1f64 / (duration / runs).as_secs_f64()) as usize,
    );
}
