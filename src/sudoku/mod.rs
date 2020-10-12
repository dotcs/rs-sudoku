use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::fmt;

mod field;
pub use field::Field;
mod solver;
pub use solver::{EnergyDimension, SolverMethod};
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

    fn is_done(&self, energy: Option<f32>) -> bool {
        // All mutable fields must be non-zero
        for field in &self.grid.mutable_fields {
            if self.grid.get(&field) == 0 {
                return false;
            }
        }

        // In case the energy is already known, prevent re-computation of the
        // energy, use the given value instead. Otherwise compute it.
        let energy = match energy {
            Some(val) => val,
            None => self.calc_energy(),
        };
        energy == 0.0
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

    pub fn solve(&mut self, method: SolverMethod, max_tries: u32) -> Result<String, String> {
        match method {
            SolverMethod::Backtracing => self.solve_backtrace(max_tries),
            SolverMethod::Montecarlo => self.solve_montecarlo(max_tries),
        }
    }

    /// Solves the sudoku by iteratively walking through all editable field with the
    /// [Backtracing](https://en.wikipedia.org/wiki/Sudoku_solving_algorithms#Backtracking)
    /// algorithm.
    /// This method is guaranteed to find a solution if the sudoku is valid.
    pub fn solve_backtrace(&mut self, max_tries: u32) -> Result<String, String> {
        let mut index = 0;
        let mut tries = 0;

        while !self.is_done(None) {
            let field = self.grid.mutable_fields[index].clone();
            let val = self.grid.get(&field);
            let guesses = self.get_field_guesses(&field);
            let next_guesses: Vec<u8> = guesses.into_iter().filter(|v| v > &val).collect();
            if next_guesses.len() == 0 {
                // No more guesses available
                // Go back one step and use next guess there
                self.grid.set(&field, 0);
                index -= 1;
            } else {
                self.grid.set(&field, next_guesses[0]);
                index += 1;
            }
            tries += 1;
            if tries == max_tries {
                return Err(format!(
                    "Could not solve sudoko. Exeeded limit of {} tries.",
                    max_tries
                ));
            }
        }

        Ok(format!("Solved. Needed {} tries.", tries))
    }

    fn random_parcel_index() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0, 9)
    }

    /// Returns all field indices (row, column) in a parcel.
    fn get_parcel_fields(parcel_index: u8) -> Vec<Field> {
        let col_start = (parcel_index % 3) * 3;
        let row_start = (parcel_index / 3) * 3;
        let mut fields: Vec<Field> = vec![];
        for r in 0..3 {
            for c in 0..3 {
                fields.push(Field::new(row_start + r, col_start + c));
            }
        }
        fields
    }

    /// Returns all field indicies (row, column) of a mutable fields in a parcel.
    fn get_mutable_fields_of_parcel(&self, parcel_index: u8) -> Vec<Field> {
        let parcel_fields = Sudoku::get_parcel_fields(parcel_index);
        parcel_fields
            .into_iter()
            .filter(|f| self.grid.mutable_fields.contains(&f))
            .collect()
    }

    /// Calculates the current energy of the system.
    /// The energy is defined as 3*n**4 minus the sum of the number of unique
    /// elements in each row, column and parcel.
    fn calc_energy(&self) -> f32 {
        let n = 3;
        let energy_max = f32::from(3 * i16::pow(n, 4));
        let mut energy: f32 = energy_max;
        for dim in [
            EnergyDimension::Column,
            EnergyDimension::Row,
            EnergyDimension::Parcel,
        ]
        .iter()
        {
            for index in 0..9 {
                energy -= f32::from(self.count_unique_elements(dim, index));
            }
        }
        energy
    }

    fn count_unique_elements(&self, dim: &EnergyDimension, index: u8) -> u8 {
        let uniq: Vec<u8> = match dim {
            EnergyDimension::Column => self.grid.get_col(index).into_iter().unique().collect(),
            EnergyDimension::Row => self.grid.get_row(index).into_iter().unique().collect(),
            EnergyDimension::Parcel => self
                .grid
                .get_parcel(index)
                .into_iter()
                .flatten()
                .unique()
                .collect(),
        };
        uniq.len() as u8
    }

    /// Solves sudoku by using a Montecarlo simulation.
    /// See details here: https://www.lptmc.jussieu.fr/user/talbot/sudoku.html
    pub fn solve_montecarlo(&mut self, max_tries: u32) -> Result<String, String> {
        let temperature = 0.15;
        let mut tries = 0;
        let mut rng = rand::thread_rng();
        let uniform_dist = Uniform::from(0.0..1.0);

        // Fill empty values with random guesses
        for pi in 0..9 {
            let mutable_fields = self.get_mutable_fields_of_parcel(pi);
            let unique_values: Vec<u8> = self
                .grid
                .get_parcel(pi)
                .into_iter()
                .flatten()
                .unique()
                .filter(|v| v > &0)
                .collect();
            let all_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let diff: Vec<u8> = all_numbers
                .into_iter()
                .filter(|v| !unique_values.contains(v))
                .collect();
            for (i, field) in mutable_fields.iter().enumerate() {
                self.grid.set(field, diff[i]);
            }
        }

        let mut energy_last = self.calc_energy();

        while !self.is_done(Some(energy_last)) {
            let rand_pi = Sudoku::random_parcel_index();
            let mut mut_fields_parcel = self.get_mutable_fields_of_parcel(rand_pi);
            mut_fields_parcel.shuffle(&mut rng);
            let f1 = &mut_fields_parcel[0];
            let f2 = &mut_fields_parcel[1];

            // Swap values
            let f1_val = self.grid.get(f1);
            let f2_val = self.grid.get(f2);
            self.grid.set(f1, f2_val);
            self.grid.set(f2, f1_val);

            let energy = self.calc_energy();
            let threshold = uniform_dist.sample(&mut rng);
            let result = ((energy_last - energy) / temperature).exp();
            let reject = result < threshold;

            if reject {
                self.grid.set(f1, f1_val);
                self.grid.set(f2, f2_val);
            } else {
                energy_last = energy;
            }

            tries += 1;
            if tries == max_tries {
                return Err(format!(
                    "Could not solve sudoko. Exeeded limit of {} tries.",
                    max_tries
                ));
            }
        }

        Ok(format!("Solved. Needed {} tries.", tries))
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
    fn it_should_list_all_parcel_fields() {
        assert_eq!(
            Sudoku::get_parcel_fields(0),
            vec![
                Field::new(0, 0),
                Field::new(0, 1),
                Field::new(0, 2),
                Field::new(1, 0),
                Field::new(1, 1),
                Field::new(1, 2),
                Field::new(2, 0),
                Field::new(2, 1),
                Field::new(2, 2)
            ]
        );
        assert_eq!(
            Sudoku::get_parcel_fields(7),
            vec![
                Field::new(6, 3),
                Field::new(6, 4),
                Field::new(6, 5),
                Field::new(7, 3),
                Field::new(7, 4),
                Field::new(7, 5),
                Field::new(8, 3),
                Field::new(8, 4),
                Field::new(8, 5)
            ]
        );
    }

    #[test]
    fn it_should_list_all_mutable_parcel_fields() {
        let mut s = Sudoku::new();
        s.read("examples/sudoku1.txt");
        let mutable_fields = s.get_mutable_fields_of_parcel(5);

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
