use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;
use std::iter;

#[derive(Debug)]
pub struct Sudoku {
    pub grid: Vec<Vec<u8>>,
    mutable_fields: Vec<(u8, u8)>,
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Sudoku::fmt(&self.grid))
    }
}

// Naming:
// valid: means that no invalid is in a row or parcel
// done: means that all (missing) values have been filled
// field: a 1x1 field
// parcel: a 3x3 field group

impl Sudoku {
    pub fn new() -> Sudoku {
        let grid = vec![vec![9]; 9];
        let mutable_fields = vec![];
        Sudoku {
            grid,
            mutable_fields,
        }
    }

    pub fn read(&mut self, file: &str) -> () {
        let content = std::fs::read_to_string(file).unwrap();
        let res: Vec<Vec<_>> = content
            .split("\n")
            .filter(|l| !l.contains("#")) // remove comments
            .filter(|l| !l.contains("-")) // remove parcel group separators
            .map(|l| {
                l.replace("|", "") // remove grid lines
                    .replace("x", "0")
                    .chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect()
            })
            .collect();
        self.grid = res;
        self.mutable_fields = self.get_mutable_fields();
    }

    pub fn get(&self, row: u8, col: u8) -> u8 {
        *self
            .grid
            .get(row as usize)
            .unwrap()
            .get(col as usize)
            .unwrap()
    }

    fn get_row(&self, row_index: u8) -> Vec<u8> {
        self.grid.get(row_index as usize).unwrap().clone()
    }

    fn is_valid_row(&self, row_index: u8) -> bool {
        let row = self.get_row(row_index);
        Sudoku::has_only_unique_digits(row)
    }

    fn get_col(&self, col_index: u8) -> Vec<u8> {
        self.grid
            .clone()
            .into_iter()
            .map(|r| r[col_index as usize])
            .collect()
    }

    fn is_valid_col(&self, col_index: u8) -> bool {
        let column = self.get_col(col_index);
        Sudoku::has_only_unique_digits(column)
    }

    fn get_parcel(&self, index: u8) -> Vec<Vec<u8>> {
        let start_row = (index / 3) * 3;
        let start_col = (index % 3) * 3;
        let mut parcel = vec![vec![0; 3]; 3];
        for ci in 0..3 {
            for ri in 0..3 {
                let row = start_row + ri;
                let col = start_col + ci;
                parcel[ri as usize][ci as usize] = self.get(row, col)
            }
        }
        parcel
    }

    fn has_only_unique_digits(digits: Vec<u8>) -> bool {
        // Get all non-zero values (unfilled values)
        let nonzero_values: Vec<u8> = digits.into_iter().filter(|v| *v != 0).collect();

        // If not all non-zero values in the parcel are unique, the parcel is not valid
        let unique_values: Vec<u8> = nonzero_values.clone().into_iter().unique().collect();

        // The parcel is valid if both, the nonzero and the unique values have the same
        // dimension
        nonzero_values.len() == unique_values.len()
    }

    fn is_valid_parcel(&self, parcel_index: u8) -> bool {
        let parcel = self.get_parcel(parcel_index);
        Sudoku::has_only_unique_digits(parcel.into_iter().flatten().collect::<Vec<u8>>())
    }

    fn get_parcel_index(row_index: u8, col_index: u8) -> u8 {
        let x = row_index / 3;
        let y = col_index / 3;
        x * 3 + y
    }

    fn is_valid_field(&self, row_index: u8, col_index: u8) -> bool {
        let parcel_index = Sudoku::get_parcel_index(row_index, col_index);
        self.is_valid_row(row_index)
            && self.is_valid_col(col_index)
            && self.is_valid_parcel(parcel_index)
    }

    fn is_valid(&self) -> bool {
        for parcel_index in 0..9 {
            if !self.is_valid_parcel(parcel_index) {
                return false;
            }
        }
        true
    }

    fn is_done(&self) -> bool {
        // The alogorithm is done if all mutable fields are non-zero and all
        // parcels are valid.

        // All mutable fields must be non-zero
        for (r, c) in &self.mutable_fields {
            if self.grid[*r as usize][*c as usize] == 0 {
                return false;
            }
        }

        // And all parcels must be valid
        self.is_valid()
    }

    fn get_mutable_fields(&self) -> Vec<(u8, u8)> {
        let mut mutable_fields: Vec<(u8, u8)> = vec![];
        for r in 0..9 {
            for c in 0..9 {
                if self.grid[r][c] == 0 {
                    mutable_fields.push((r as u8, c as u8));
                }
            }
        }
        mutable_fields
    }

    fn get_field_guesses(&self, row_index: u8, col_index: u8) -> Vec<u8> {
        let mut set_allowed: HashSet<u8> = HashSet::new();
        for i in 1..10 {
            set_allowed.insert(i);
        }

        let mut seen: HashSet<u8> = HashSet::new();
        let values_row: Vec<u8> = self.get_row(row_index);
        let values_col: Vec<u8> = self.get_col(col_index);
        let values_parcel: Vec<u8> = self
            .get_parcel(Sudoku::get_parcel_index(row_index, col_index))
            .into_iter()
            .flatten()
            .collect();
        seen.extend(values_row);
        seen.extend(values_col);
        seen.extend(values_parcel);

        let mut guesses: Vec<u8> = set_allowed.difference(&seen).map(|x| *x).collect();
        guesses.sort();
        guesses
    }

    pub fn solve(&mut self) {
        let mut index = 0;
        let mut tries = 0;

        while !self.is_done() {
            let (r, c) = self.mutable_fields[index];
            let val = self.grid[r as usize][c as usize];
            let guesses = self.get_field_guesses(r, c);
            let next_guesses: Vec<u8> = guesses.into_iter().filter(|v| v > &val).collect();
            if next_guesses.len() == 0 {
                // No more guesses available
                // Go back one step and use next guess there
                self.grid[r as usize][c as usize] = 0;
                index -= 1;
            } else {
                self.grid[r as usize][c as usize] = next_guesses[0];
                index += 1;
            }
            tries += 1;
        }

        println!("Solved. Needed {} tries.", tries);
    }

    pub fn reset(&mut self) {
        for (r, c) in self.mutable_fields.iter() {
            self.grid[*r as usize][*c as usize] = 0;
        }
    }

    pub fn get_unsolved(&self) -> Vec<Vec<u8>> {
        let mut grid_copy = self.grid.clone();
        for (r, c) in self.mutable_fields.iter() {
            grid_copy[*r as usize][*c as usize] = 0;
        }
        grid_copy
    }

    pub fn fmt(grid: &Vec<Vec<u8>>) -> String {
        let mut out = String::new();
        for (i, row) in (grid).iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                out += &iter::repeat("-").take(11).collect::<String>()[..];
                out += "\n";
            }
            for (j, v) in row.iter().enumerate() {
                if j > 0 && j % 3 == 0 {
                    out += "|";
                }
                if v == &0 {
                    out += "x";
                } else {
                    let val = format!("{}", v);
                    out += &val[..];
                }
            }
            if i < grid.len() - 1 {
                out += "\n";
            }
        }
        out
    }

    pub fn print(grid: &Vec<Vec<u8>>) {
        println!("{}", Sudoku::fmt(grid));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_read_file() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
    }

    #[test]
    fn it_should_get_row_col_values() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert_eq!(s.get(0, 6), 7);
        assert_eq!(s.get(1, 6), 4);
    }

    #[test]
    fn it_should_get_parcels() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert_eq!(
            s.get_parcel(0),
            vec![vec![4, 3, 5], vec![6, 8, 2], vec![1, 9, 7]]
        );
        assert_eq!(
            s.get_parcel(3),
            vec![vec![8, 2, 6], vec![3, 7, 4], vec![9, 5, 1]]
        );
    }

    #[test]
    fn it_should_test_parcel_validity() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert!(s.is_valid_parcel(0));

        s.grid[0][0] = 1;
        s.grid[0][1] = 1;
        assert!(!s.is_valid_parcel(0));
    }

    #[test]
    fn it_should_give_rows() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert_eq!(s.get_row(2), vec![1, 9, 7, 8, 3, 4, 5, 6, 2]);
    }

    #[test]
    fn it_should_give_columns() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert_eq!(s.get_col(2), vec![5, 2, 7, 6, 4, 1, 9, 8, 3]);
    }

    #[test]
    fn it_should_be_valid() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert!(s.is_valid());

        s.grid[0][0] = 6;
        assert!(!s.is_valid());
    }

    #[test]
    fn it_should_flag_solution_as_done() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert!(s.is_done());
    }

    #[test]
    fn it_should_flag_unsolved_sudoko_as_not_done() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1.txt");
        assert!(!s.is_done());
    }

    #[test]
    fn it_should_mark_mutable_fields() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1.txt");

        assert_eq!(
            s.mutable_fields,
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 5),
                (0, 7),
                (1, 2),
                (1, 3),
                (1, 5),
                (1, 6),
                (1, 8),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 7),
                (2, 8),
                (3, 2),
                (3, 4),
                (3, 5),
                (3, 6),
                (3, 8),
                (4, 0),
                (4, 1),
                (4, 4),
                (4, 7),
                (4, 8),
                (5, 0),
                (5, 2),
                (5, 3),
                (5, 4),
                (5, 6),
                (6, 0),
                (6, 1),
                (6, 4),
                (6, 5),
                (6, 6),
                (7, 0),
                (7, 2),
                (7, 3),
                (7, 5),
                (7, 6),
                (8, 1),
                (8, 3),
                (8, 6),
                (8, 7),
                (8, 8)
            ]
        );
    }

    #[test]
    fn it_should_have_correct_field_guesses() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1.txt");
        assert_eq!(s.get_field_guesses(0, 0), vec![3, 4, 5]);
        assert_eq!(s.get_field_guesses(8, 8), vec![2, 5, 9]);
    }
    #[test]
    fn it_should_reset_values() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1.txt");

        // Sanity check; (0,0) must be mutable field
        assert_eq!(s.get_mutable_fields()[0], (0, 0));

        s.grid[0][0] = 5; // change value so that there is something to reset
        assert_eq!(s.grid[0][0], 5);

        s.reset();
        assert_eq!(s.grid[0][0], 0);
    }
}