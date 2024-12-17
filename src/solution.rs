use std::fmt::{Display, Formatter};
use std::path::Path;
use std::fs;

pub struct Solution([[u8; 9]; 9]);

impl Solution {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let contents = fs::read_to_string(path)
            .expect("File read error");

        let mut board = [[9; 9]; 9];

        for (i, line) in contents.lines().enumerate() {
            if i > 8 && !line.trim().is_empty() {
                panic!("Too many lines");
            }
            else if i > 8 {
                continue;
            }
            for (j, c) in line.chars().enumerate() {
                if j > 8 && !c.to_string().is_empty() { panic!("Line too long in input"); }
                else if j > 8 {
                    continue;
                }
                if c == '_' || c == '0' {
                    continue;
                }
                else {
                    let n = c.to_digit(10);
                    if let Some(n) = n {
                        board[i][j] = n as u8 - 1;
                    }
                    else { panic!("Expected number, found {c}") }
                }
            }
        }

        Solution(board)
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.0[y][x]
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌───────┬───────┬───────┐")?;
        for row in 0..9 {
            write!(f, "│ ")?;
            for col in 0..9 {
                if let Some(n) = self.get(col, row) != 9 {
                    write!(f, "{}", n+1)?;
                }
                else {
                    write!(f, "-")?;
                }

                if (col + 1) % 3 == 0 && col != 8 {
                    write!(f, " │ ")?;
                }
                else {
                    write!(f, " ")?;
                }

            }
            writeln!(f, "│")?;

            if (row + 1) % 3 == 0 && row != 8 {
                writeln!(f, "├───────┼───────┼───────┤")?;
            }
        }
        writeln!(f, "└───────┴───────┴───────┘")?;

        Ok(())
    }
}