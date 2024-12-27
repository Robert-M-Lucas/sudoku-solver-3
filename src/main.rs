use std::cmp::max;
use crate::solver::solve_backtracking;
use std::hint::black_box;
use std::io::{stdout, Write};
use std::time::Instant;
use color_print::{cprint, cprintln};
use itertools::Itertools;
use rayon::prelude::*;
use thousands::Separable;

mod board;
mod solution;
mod solver;
mod util;

const INDIVIDUAL_RUNS: usize = 100_000;
const MIXED_RUNS: usize = 100_000;
const INDIVIDUAL_PAR_RUNS: usize = 1_000_000;
const MIXED_PAR_RUNS: usize = 1_000_000;


fn main() {
    let mut puzzles = Vec::new();

    cprintln!("<b, bold>Loading puzzles...");
    for name in ["very_easy", "easy", "medium", "hard"] {
        puzzles.push((util::title_case(name), util::load_puzzles(name)))
    }
    let longest_name = puzzles.iter().fold(0, |c, (n, _)| max(n.chars().count(), c));
    cprintln!("<g, bold>Done");

    cprint!("<b, bold>Validating solver... ");

    for (name, puzzles) in &puzzles {
        let name = format!("{:>width$}", name, width=longest_name);
        for (i, (puzzle, solution)) in puzzles.iter().enumerate() {
            let s = solve_backtracking(puzzle.clone());
            let solution = solution.clone();
            if s != solution {
                cprintln!("<r, bold>\nFailed to solve {name} - {i}:");
                if let Some(s) = s {
                    println!("Given solution:\n{s}")
                }
                else {
                    println!("No solution found")
                }

                if let Some(solution) = solution {
                    println!("Actual solution:\n{solution}")
                }
                else {
                    println!("No actual solution")
                }
            }
        }
    }

    cprintln!("<g, bold>Done");

    cprintln!("<b, bold>Measuring performance...");

    for (name, puzzles) in &puzzles {
        let name = format!("{:<width$}", name, width=longest_name);
        let puzzles = puzzles.iter().map(|(s, _)| s.clone()).collect_vec();
        let total_solved = INDIVIDUAL_RUNS * puzzles.len();
        cprint!("<b, bold>Timing {name} [{} runs | {} puzzles]", INDIVIDUAL_RUNS.separate_with_commas(), total_solved.separate_with_commas());
        stdout().flush().ok();
        let start = Instant::now();
        for _ in 0..INDIVIDUAL_RUNS {
            for puzzle in &puzzles {
                let s = solve_backtracking(puzzle.clone());
                black_box(s);
            }
        }
        let duration = start.elapsed();

        cprintln!("\r<g, bold>{name} $ Total solved: {} | Duration: {:?} | Time per: {:?} | Per second: {}",
            total_solved.separate_with_commas(),
            duration,
            duration / total_solved as u32,
            (1f64 / (duration / total_solved as u32).as_secs_f64()) as usize,
        );
    }

    let a_puzzles = puzzles.iter().map(|(_, v)| v).flatten().map(|(p, _)| p.clone()).collect_vec();
    let total_solved = MIXED_RUNS * a_puzzles.len();
    cprint!("<b, bold>Timing mixed [{} runs | {} puzzles]", MIXED_RUNS.separate_with_commas(), total_solved.separate_with_commas());
    stdout().flush().ok();
    let start = Instant::now();
    for _ in 0..MIXED_RUNS {
        for puzzle in &a_puzzles {
            let s = solve_backtracking(puzzle.clone());
            black_box(s);
        }
    }
    let duration = start.elapsed();

    let all_text = format!("{:<width$}", "All", width=longest_name);
    cprintln!("\r<g, bold>{} $ Total solved: {} | Duration: {:?} | Time per: {:?} | Per second: {}",
        all_text,
        total_solved.separate_with_commas(),
        duration,
        duration / total_solved as u32,
        (1f64 / (duration / total_solved as u32).as_secs_f64()) as usize,
    );

    cprintln!("<b, bold>{:-^101}", "Parallel Tests");

    for (name, puzzles) in &puzzles {
        let name = format!("{:<width$}", name, width=longest_name);
        let puzzles = puzzles.iter().map(|(s, _)| s.clone()).collect_vec();
        let total_solved = INDIVIDUAL_PAR_RUNS * puzzles.len();
        cprint!("<b, bold>Timing {name} [{} runs | {} puzzles]", INDIVIDUAL_PAR_RUNS.separate_with_commas(), total_solved.separate_with_commas());
        stdout().flush().ok();
        let start = Instant::now();
        let mut x = vec![1, 2, 3];
        (0..INDIVIDUAL_PAR_RUNS).into_par_iter().for_each(|_| {
            for puzzle in &puzzles {
                let s = solve_backtracking(puzzle.clone());
                black_box(s);
            }
        });

        let duration = start.elapsed();

        cprintln!("\r<g, bold>{name} $ Total solved: {} | Duration: {:?} | Time per: {:?} | Per second: {}",
            total_solved.separate_with_commas(),
            duration,
            duration / total_solved as u32,
            (1f64 / (duration / total_solved as u32).as_secs_f64()) as usize,
        );
    }

    let a_puzzles = puzzles.iter().map(|(_, v)| v).flatten().map(|(p, _)| p.clone()).collect_vec();
    let total_solved = MIXED_PAR_RUNS * a_puzzles.len();
    cprint!("<b, bold>Timing mixed [{} runs | {} puzzles]", MIXED_PAR_RUNS.separate_with_commas(), total_solved.separate_with_commas());
    stdout().flush().ok();
    let start = Instant::now();
    (0..MIXED_PAR_RUNS).into_par_iter().for_each(|_| {
        for puzzle in &a_puzzles {
            let s = solve_backtracking(puzzle.clone());
            black_box(s);
        }
    });
    let duration = start.elapsed();

    let all_text = format!("{:<width$}", "All", width=longest_name);
    cprintln!("\r<g, bold>{} $ Total solved: {} | Duration: {:?} | Time per: {:?} | Per second: {}",
        all_text,
        total_solved.separate_with_commas(),
        duration,
        duration / total_solved as u32,
        (1f64 / (duration / total_solved as u32).as_secs_f64()) as usize,
    );
}
