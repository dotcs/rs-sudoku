use std::collections::HashSet;
use std::fmt;

mod field;
pub mod solver;
pub use field::Field;
mod common;
mod grid;
pub use grid::Grid;

#[derive(Debug)]
pub struct Sudoku {
    pub grid: Grid,
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.grid.fmt())
    }
}

// Naming:
// valid:  means that no duplicated values are in a row or parcel (with the
//         exception of the value 0)
// done:   means that all (missing) values have been filled
// field:  a 1x1 field in the grid
// parcel: a 3x3 field group (numbered 0 - 8, row major)

impl Sudoku {
    /// Creates a new sudoku instance.
    /// Make sure to run the `read` method afterwards to read a sudoku from a
    /// file.
    pub fn new() -> Sudoku {
        let grid = Grid::new(vec![vec![0; 9]; 9]);
        Sudoku { grid }
    }

    /// Reads a sudoku from a file.
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
        self.grid = Grid::new(res);
    }

    #[allow(dead_code)]
    pub fn is_valid_row(&self, row_index: u8) -> bool {
        let row = self.grid.get_row(row_index);
        common::has_only_unique_digits(&row)
    }

    #[allow(dead_code)]
    pub fn is_valid_col(&self, col_index: u8) -> bool {
        let column = self.grid.get_col(col_index);
        common::has_only_unique_digits(&column)
    }

    #[allow(dead_code)]
    pub fn is_valid_parcel(&self, parcel_index: u8) -> bool {
        let parcel = self.grid.get_parcel(parcel_index);
        common::has_only_unique_digits(&parcel.into_iter().flatten().collect::<Vec<u8>>())
    }

    #[allow(dead_code)]
    fn is_valid_field(&self, field: &Field) -> bool {
        let parcel_index = Grid::get_parcel_index(&field);
        self.is_valid_row(field.row)
            && self.is_valid_col(field.column)
            && self.is_valid_parcel(parcel_index)
    }

    #[allow(dead_code)]
    fn is_valid(&self) -> bool {
        for parcel_index in 0..9 {
            if !self.is_valid_parcel(parcel_index) {
                return false;
            }
        }
        true
    }

    /// Simple implementation to test if the sudoku has been solved.
    /// This implementation only checks if any field is zero and if all parcels
    /// are valid, which means each parcel only has values from 1 - 9.
    /// It does not test if any row or column contain duplicate values.
    fn is_done(&self) -> bool {
        let any_zero = self
            .grid
            .mutable_fields
            .iter()
            .any(|field| self.grid.get(field) == 0);
        if any_zero {
            return false;
        }

        self.is_valid()
    }

    fn get_field_guesses(&self, field: &Field) -> Vec<u8> {
        let mut set_allowed: HashSet<u8> = HashSet::new();
        for i in 1..10 {
            set_allowed.insert(i);
        }

        let mut seen: HashSet<u8> = HashSet::new();
        let values_row: Vec<u8> = self.grid.get_row(field.row);
        let values_col: Vec<u8> = self.grid.get_col(field.column);
        let values_parcel: Vec<u8> = self
            .grid
            .get_parcel(Grid::get_parcel_index(field))
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

    /// Returns a grid in its unsolved representation. Every editable field
    /// is set to 0.
    pub fn get_unsolved(&self) -> Grid {
        let mut grid_copy = self.grid.clone();
        grid_copy.reset();
        grid_copy
    }

    /// Prints the sudoku to stdout.
    /// This function uses `Sudoku::fmt` for the formatting of the sudoku.
    /// If `show_unresolved` is set to `true` the unsolved sudoku is shown next
    /// to the solved one.
    pub fn print(&self, show_unsolved: bool) {
        if !show_unsolved {
            println!("{}", Grid::fmt(&self.grid));
        } else {
            let unresolved = Grid::fmt(&self.get_unsolved());
            let solved = Grid::fmt(&self.grid);
            let solved_iter: Vec<&str> = solved.split("\n").collect();
            for (i, line) in unresolved.split("\n").enumerate() {
                println!("{} -> {}", line, solved_iter.get(i).unwrap());
            }
        }
    }
}

impl std::clone::Clone for Sudoku {
    fn clone(&self) -> Sudoku {
        Sudoku {
            grid: self.grid.clone(),
        }
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
        assert_eq!(s.grid.get(&Field::new(0, 6)), 7);
        assert_eq!(s.grid.get(&Field::new(1, 6)), 4);
    }

    #[test]
    fn it_should_get_parcels() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert_eq!(
            s.grid.get_parcel(0),
            vec![vec![4, 3, 5], vec![6, 8, 2], vec![1, 9, 7]]
        );
        assert_eq!(
            s.grid.get_parcel(3),
            vec![vec![8, 2, 6], vec![3, 7, 4], vec![9, 5, 1]]
        );
    }

    #[test]
    fn it_should_test_parcel_validity() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert!(s.is_valid_parcel(0));

        s.grid.set(&Field::new(0, 0), 1);
        s.grid.set(&Field::new(0, 1), 1);
        assert!(!s.is_valid_parcel(0));
    }

    #[test]
    fn it_should_give_rows() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert_eq!(s.grid.get_row(2), vec![1, 9, 7, 8, 3, 4, 5, 6, 2]);
    }

    #[test]
    fn it_should_give_columns() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert_eq!(s.grid.get_col(2), vec![5, 2, 7, 6, 4, 1, 9, 8, 3]);
    }

    #[test]
    fn it_should_be_valid() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert!(s.is_valid());

        s.grid.set(&Field::new(0, 0), 6);
        assert!(!s.is_valid());
    }

    #[test]
    fn it_should_flag_solution_as_done() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1-solution.txt");
        assert!(s.is_done(None));
    }

    #[test]
    fn it_should_flag_unsolved_sudoko_as_not_done() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1.txt");
        assert!(!s.is_done(None));
    }

    #[test]
    fn it_should_mark_mutable_fields() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1.txt");

        assert_eq!(
            s.grid.mutable_fields,
            vec![
                Field::new(0, 0),
                Field::new(0, 1),
                Field::new(0, 2),
                Field::new(0, 5),
                Field::new(0, 7),
                Field::new(1, 2),
                Field::new(1, 3),
                Field::new(1, 5),
                Field::new(1, 6),
                Field::new(1, 8),
                Field::new(2, 2),
                Field::new(2, 3),
                Field::new(2, 4),
                Field::new(2, 7),
                Field::new(2, 8),
                Field::new(3, 2),
                Field::new(3, 4),
                Field::new(3, 5),
                Field::new(3, 6),
                Field::new(3, 8),
                Field::new(4, 0),
                Field::new(4, 1),
                Field::new(4, 4),
                Field::new(4, 7),
                Field::new(4, 8),
                Field::new(5, 0),
                Field::new(5, 2),
                Field::new(5, 3),
                Field::new(5, 4),
                Field::new(5, 6),
                Field::new(6, 0),
                Field::new(6, 1),
                Field::new(6, 4),
                Field::new(6, 5),
                Field::new(6, 6),
                Field::new(7, 0),
                Field::new(7, 2),
                Field::new(7, 3),
                Field::new(7, 5),
                Field::new(7, 6),
                Field::new(8, 1),
                Field::new(8, 3),
                Field::new(8, 6),
                Field::new(8, 7),
                Field::new(8, 8)
            ]
        );
    }

    #[test]
    fn it_should_have_correct_field_guesses() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1.txt");
        assert_eq!(s.get_field_guesses(&Field::new(0, 0)), vec![3, 4, 5]);
        assert_eq!(s.get_field_guesses(&Field::new(8, 8)), vec![2, 5, 9]);
    }
    #[test]
    fn it_should_reset_values() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1.txt");

        // Sanity check; (0,0) must be mutable field
        assert_eq!(s.grid.mutable_fields[0], Field::new(0, 0));

        s.grid.set(&Field::new(0, 0), 5); // change value so that there is something to reset
        assert_eq!(s.grid.get(&Field::new(0, 0)), 5);

        s.grid.reset();
        assert_eq!(s.grid.get(&Field::new(0, 0)), 0);
    }

    #[test]
    fn it_should_list_all_mutable_parcel_fields() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1.txt");
        let mutable_fields = s.grid.get_mutable_fields_of_parcel(5);

        assert_eq!(
            mutable_fields,
            vec![
                Field::new(3, 6),
                Field::new(3, 8),
                Field::new(4, 7),
                Field::new(4, 8),
                Field::new(5, 6),
            ]
        );
    }
}
