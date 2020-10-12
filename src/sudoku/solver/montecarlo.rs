use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::Rng;

use super::super::Sudoku;
use super::Solver;

pub enum EnergyDimension {
    Row,
    Column,
    Parcel,
}

impl Sudoku {
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

    fn is_done_with_energy(&self, energy: Option<f32>) -> bool {
        if !self.is_done() {
            return false;
        }

        // In case the energy is already known, prevent re-computation of the
        // energy, use the given value instead. Otherwise compute it.
        let energy = match energy {
            Some(val) => val,
            None => self.calc_energy(),
        };
        energy == 0.0
    }
}

pub struct Montecarlo {
    max_tries: u32,
    tries: u32,
    temperature: f32,
    rng: rand::prelude::ThreadRng,
}

impl Montecarlo {
    pub fn new(max_tries: u32, temperature: f32) -> Montecarlo {
        Montecarlo {
            max_tries,
            temperature,
            tries: 0,
            rng: rand::thread_rng(),
        }
    }
}

impl Solver for Montecarlo {
    fn is_success(&self) -> bool {
        self.tries < self.max_tries
    }

    fn get_tries(&self) -> u32 {
        self.tries
    }

    /// Solves sudoku by using a Montecarlo simulation.
    /// See details here: https://www.lptmc.jussieu.fr/user/talbot/sudoku.html
    fn solve(&mut self, mut sudoku: Sudoku) -> Sudoku {
        let uniform_dist = Uniform::from(0.0..1.0);

        // Fill empty values with random guesses
        for pi in 0..9 {
            let mutable_fields = sudoku.grid.get_mutable_fields_of_parcel(pi);
            let unique_values: Vec<u8> = sudoku
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
                sudoku.grid.set(field, diff[i]);
            }
        }

        let mut energy_last = sudoku.calc_energy();

        while !sudoku.is_done_with_energy(Some(energy_last)) {
            let rand_pi = self.rng.gen_range(0, 9);
            let mut mut_fields_parcel = sudoku.grid.get_mutable_fields_of_parcel(rand_pi);
            mut_fields_parcel.shuffle(&mut self.rng);
            let f1 = &mut_fields_parcel[0];
            let f2 = &mut_fields_parcel[1];

            // Swap values
            let f1_val = sudoku.grid.get(f1);
            let f2_val = sudoku.grid.get(f2);
            sudoku.grid.set(f1, f2_val);
            sudoku.grid.set(f2, f1_val);

            let energy = sudoku.calc_energy();
            let threshold = uniform_dist.sample(&mut self.rng);
            let result = ((energy_last - energy) / self.temperature).exp();
            let reject = result < threshold;

            if reject {
                sudoku.grid.set(f1, f1_val);
                sudoku.grid.set(f2, f2_val);
            } else {
                energy_last = energy;
            }

            self.tries += 1;
            if self.tries >= self.max_tries {
                break;
            }
        }

        sudoku
    }
}
