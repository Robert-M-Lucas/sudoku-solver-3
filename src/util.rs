use std::fs;
use crate::solution::Solution;

pub fn load_puzzles(name: &str) -> Vec<(Solution, Option<Solution>)> {
    let mut puzzles = Vec::new();
    let mut i: usize = 0;
    loop {
        let Ok(ps) = fs::read_to_string(format!("converted/{name}_{i}_puzzle.txt")) else { break; };
        let Ok(ss) = fs::read_to_string(format!("converted/{name}_{i}_solution.txt")) else { break; };

        if ss.chars().next().unwrap() != '!' {
            puzzles.push(
                (Solution::load_string(ps), Some(Solution::load_string(ss)))
            );
        }
        else {
            puzzles.push(
                (Solution::load_string(ps), None)
            );
        }


        i += 1;
    }

    println!("Loaded {i} {name} puzzles");

    puzzles
}

pub fn title_case(name: &str) -> String {
    let mut sb = String::new();
    let mut prev_space = true;
    for c in name.chars() {
        let c = if c == '_' { ' ' } else { c };
        if c == ' ' {
            prev_space = true;
            sb.push(c);
            continue;
        }

        if prev_space {
            sb.push(c.to_ascii_uppercase());
            prev_space = false;
            continue;
        }

        sb.push(c);
    }
    sb
}