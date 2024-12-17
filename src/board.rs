use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use std::fs;
use itertools::{iproduct, Itertools};

#[derive(Clone, Copy)]
pub struct SudokuPossibility(u16);

impl SudokuPossibility {
    #[inline]
    pub const fn mask(self) -> u16 {
        self.0
    }

    #[inline]
    pub const fn new() -> Self {
        SudokuPossibility(0b00000001_11111111)
    }

    #[inline]
    pub const fn new_val(val: u8) -> Self {
        SudokuPossibility(1 << val)
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub const fn has(self, val: u8) -> bool {
        self.0 & (1 << val) != 0
    }

    #[inline]
    pub const fn and_mask(self, mask: u16) -> SudokuPossibility {
        SudokuPossibility(self.0 & mask)
    }

    #[inline]
    pub const fn and_mask_inplace(&mut self, mask: u16) {
        self.0 &= mask;
    }

    pub fn slow_find(self) -> Option<u8> {
        let mut found = 9;
        for i in 0..9 {
            if self.has(i) {
                if found != 9 {
                    return None;
                }
                else {
                    found = i;
                }
            }
        }

        if found != 9 {
            Some(found)
        }
        else {
            None
        }
    }

}

impl Display for SudokuPossibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sb = String::with_capacity(6*3);
        let mut found = false;

        for a in 0..3u8 {
            for b in 0..3u8 {
                let n = (a * 3) + b;
                if self.has(n) {
                    found = true;
                    sb += &format!("{}", n+1);
                }
                else {
                    sb.push(' ');
                }
                if b != 2 {
                    sb.push(' ');
                }
            }
            if a != 2 {
                sb.push('\n');
            }
        }

        if found {
            write!(f, "{sb}")?;
        }
        else {
            write!(f, "X X X\nX X X\nX X X")?;
        }

        Ok(())
    }
}

pub struct Board([[SudokuPossibility; 9]; 9]);

impl Board {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let contents = fs::read_to_string(path)
            .expect("File read error");

        let board = [[SudokuPossibility::new(); 9]; 9];
        let mut b = Board(board);

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
                        b.update_found(j, i, n as u8 - 1);
                    }
                    else { panic!("Expected number, found {c}") }
                }
            }
        }

        b
    }

    pub fn update_found(&mut self, x: usize, y: usize, val: u8) {
        debug_assert!(!self.0[y][x].and_mask(SudokuPossibility::new_val(val).mask()).is_empty());

        let mask: u16 = !(1 << val);

        let (cell_x, cell_y) = ((x / 3) * 3, (y / 3) * 3);
        for (yy, xx) in iproduct!(0..3, 0..3) {
            self.0[cell_y + yy][cell_x + xx].and_mask_inplace(mask);
        }

        for xx in 0..9 {
            self.0[y][xx].and_mask_inplace(mask);
        }

        for yy in  0..9 {
            self.0[yy][x].and_mask_inplace(mask);
        }

        self.0[y][x] = SudokuPossibility::new_val(val);
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strings = self.0.iter().map(|r|
            r.iter().map(|c| c.to_string()).collect_vec()
        ).collect_vec();

        writeln!(f, "╔═══════════════════════╦═══════════════════════╦═══════════════════════╗")?;
        for y in 0..9 {
            for yy in 0..3 {
                write!(f, "║ ")?;
                for x in 0..9 {
                    write!(f, "{}", strings[y][x].lines().nth(yy).unwrap())?;
                    if x == 8 {}
                    else if (x + 1) % 3 == 0 {
                        write!(f, " ║ ")?;
                    }
                    else {
                        write!(f, " │ ")?;
                    }
                }
                write!(f, " ║\n")?;
            }

            if y == 8 {}
            else if (y + 1) % 3 == 0 {
                write!(f, "╠═══════════════════════╬═══════════════════════╬═══════════════════════╣\n")?;
            }
            else {
                write!(f, "╠───────┼───────┼───────╬───────┼───────┼───────╬───────┼───────┼───────╣\n")?;
            }
        }
        write!(f, "╚═══════════════════════╩═══════════════════════╩═══════════════════════╝")?;

        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌───────┬───────┬───────┐")?;
        for row in 0..9 {
            write!(f, "│ ")?;
            for col in 0..9 {
                if let Some(n) = self.0[row][col].slow_find() {
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