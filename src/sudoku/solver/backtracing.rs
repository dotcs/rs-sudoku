use super::super::Sudoku;
use super::Solver;

pub struct Backtracing {
    max_tries: u32,
    tries: u32,
}

impl Backtracing {
    pub fn new(max_tries: u32) -> Backtracing {
        Backtracing {
            max_tries,
            tries: 0,
        }
    }
}

impl Solver for Backtracing {
    fn is_success(&self) -> bool {
        self.tries < self.max_tries
    }

    fn get_tries(&self) -> u32 {
        self.tries
    }

    /// Solves the sudoku by iteratively walking through all editable field with the
    /// [Backtracing](https://en.wikipedia.org/wiki/Sudoku_solving_algorithms#Backtracking)
    /// algorithm.
    /// This method is guaranteed to find a solution if the sudoku is valid.
    fn solve(&mut self, mut sudoku: Sudoku) -> Sudoku {
        let mut index = 0;

        while !sudoku.is_done() {
            let field = sudoku.grid.mutable_fields[index].clone();
            let val = sudoku.grid.get(&field);
            let guesses = sudoku.get_field_guesses(&field);
            let next_guesses: Vec<u8> = guesses.into_iter().filter(|v| v > &val).collect();
            if next_guesses.len() == 0 {
                // No more guesses available
                // Go back one step and use next guess there
                sudoku.grid.set(&field, 0);
                index -= 1;
            } else {
                sudoku.grid.set(&field, next_guesses[0]);
                index += 1;
            }
            self.tries += 1;
            if self.tries >= self.max_tries {
                break;
            }
        }

        sudoku
    }
}
