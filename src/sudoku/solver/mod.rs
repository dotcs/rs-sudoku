pub mod backtracing;
pub mod montecarlo;

use super::super::sudoku::Sudoku;

pub trait Solver {
    fn is_success(&self) -> bool;
    fn get_tries(&self) -> u32;
    fn solve(&mut self, sudoku: Sudoku) -> Sudoku;
}

pub use backtracing::Backtracing;
pub use montecarlo::Montecarlo;
