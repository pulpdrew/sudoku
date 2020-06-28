use crate::{
    nine_by_nine::NineByNine,
    nine_set::{union, NineSet},
};
use std::fmt;

/// A Sudoku puzzle.
#[derive(Clone)]
pub struct SudokuPuzzle {
    nums: NineByNine<u8>,
}

impl SudokuPuzzle {
    /// Create a new Puzzle from the given string.
    ///
    /// The string should consist of 81 numbers in the range [0,9],
    /// where '0' indicates an empty space and '1'-'9' represent a
    /// filled spaces with the given number.
    pub fn from_string(source: &str) -> Self {
        let source_nums = source
            .lines()
            .map(|l| l.trim())
            .flat_map(|l| l.chars())
            .map(|c| c.to_digit(10).unwrap() as u8)
            .map(|n| if n == 0 { None } else { Some(n) })
            .collect::<Vec<Option<u8>>>();

        let mut nums = NineByNine::new();
        for row in 0..9 {
            for col in 0..9 {
                nums.set(row, col, source_nums[row * 9 + col]);
            }
        }

        SudokuPuzzle { nums }
    }

    /// Indicates whether this puzzle is correctly solved
    pub fn is_solved(&self) -> bool {
        self.count_unfilled() == 0 && self.is_consistent()
    }

    /// Solve this puzzle, if possible, filling in any unfilled spaces.
    pub fn solve(&self) -> Option<SudokuPuzzle> {
        let solution = self.fill_all();

        if let Some(solution) = solution {
            if solution.is_solved() {
                Some(solution)
            } else if solution.is_consistent() {
                solution.try_guesses()
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Find a square that could be filled multiple ways. Try each
    /// choice and return the first that leads to a valid solution.
    /// return None if no choice leads to a valid solution.
    fn try_guesses(&self) -> Option<SudokuPuzzle> {
        let could_be_sets = self.could_be_sets();

        // Find a square that could be filled multiple ways
        let (mut row, mut col) = (0, 0);
        for r in 0..9 {
            for c in 0..9 {
                if could_be_sets.get(r, c).unwrap().size() > 1 {
                    row = r;
                    col = c;
                }
            }
        }
        let guesses = could_be_sets.get(row, col);

        // Try each guess, recursively attempting to solve the puzzle that
        // results from making that guess, until some guess yields a solution.
        for guess in guesses.unwrap().to_vec().iter() {
            let mut puzzle_guess = self.clone();
            puzzle_guess.nums.set(row, col, Some(*guess));
            if let Some(solution) = puzzle_guess.solve() {
                return Some(solution);
            }
        }

        None
    }

    /// Fill every index that can be filled by iterative deduction.
    /// Return `None` if some square could never be filled
    fn fill_all(&self) -> Option<SudokuPuzzle> {
        let mut prev_unfilled = self.nums.count_nones();
        let mut filled = self.fill_once();

        while filled.is_some() && filled.as_ref().unwrap().count_unfilled() != prev_unfilled {
            prev_unfilled = filled.as_ref().unwrap().count_unfilled();
            filled = filled.as_ref().unwrap().fill_once();
        }

        filled
    }

    /// Do one pass of the puzzle and fill any numbers that can be deduced.
    /// Return `None` if some square could never be filled while maintaining
    /// consistency with the other squares that have already been filled.
    fn fill_once(&self) -> Option<SudokuPuzzle> {
        let could_be_sets = self.could_be_sets();

        let mut nums = [None; 81];
        for row in 0..9 {
            for col in 0..9 {
                let could_be = could_be_sets.get(row, col).unwrap();
                if could_be.size() == 1 {
                    nums[row * 9 + col] = Some(could_be.to_vec()[0]);
                } else if could_be.size() == 0 {
                    return None;
                }
            }
        }

        Some(SudokuPuzzle {
            nums: NineByNine::from(nums),
        })
    }

    /// Generate the sets of numbers that each index could be
    fn could_be_sets(&self) -> NineByNine<NineSet> {
        let mut sets = [None; 81];
        for row in 0..9 {
            for col in 0..9 {
                sets[row * 9 + col] = if let Some(n) = self.nums.get(row, col) {
                    let mut set = NineSet::empty();
                    set.add(*n);
                    Some(set)
                } else {
                    Some(self.could_be_set(row, col))
                }
            }
        }
        NineByNine::from(sets)
    }

    /// Generate the set of numbers that the given index could be
    fn could_be_set(&self, row: usize, col: usize) -> NineSet {
        union(vec![
            self.row_set(row),
            self.col_set(col),
            self.sqr_set((row / 3 * 3) + (col / 3)),
        ])
        .complement()
    }

    /// Indicates whether this puzzle is consistent, that is,
    /// it does not violate the the rules of Sudoku.
    fn is_consistent(&self) -> bool {
        for i in 0..9 {
            let row = self.row_list(i);
            let col = self.col_list(i);
            let sqr = self.sqr_list(i);

            if row.len() != NineSet::from(row).size()
                || col.len() != NineSet::from(col).size()
                || sqr.len() != NineSet::from(sqr).size()
            {
                return false;
            }
        }
        true
    }

    /// Returns the number of squares in this puzzle that
    /// have not been filled with a number already.
    fn count_unfilled(&self) -> usize {
        self.nums.count_nones()
    }

    /// The list of numbers (with any repeats) in the row with the given index.
    fn row_list(&self, row_idx: usize) -> Vec<u8> {
        assert!(row_idx < 9);

        let mut row_list = Vec::with_capacity(9);
        for col in 0..9 {
            if let Some(n) = self.nums.get(row_idx, col) {
                row_list.push(*n);
            }
        }
        row_list
    }

    /// The list of numbers (with any repeats) in the square with the given index.
    fn col_list(&self, col_idx: usize) -> Vec<u8> {
        assert!(col_idx < 9);

        let mut col_list = Vec::with_capacity(9);
        for row in 0..9 {
            if let Some(n) = self.nums.get(row, col_idx) {
                col_list.push(*n);
            }
        }
        col_list
    }

    /// The list of numbers (with any repeats) in the square with the given index.
    /// Indices are in the range [0,9), begin in the upper left hand corner
    /// of the puzzle, and proceed left to right, top to bottom.
    fn sqr_list(&self, sqr_idx: usize) -> Vec<u8> {
        assert!(sqr_idx < 9);

        let mut sqr_list = Vec::with_capacity(9);
        for i in 0..9 {
            let row = sqr_idx / 3 * 3 + i / 3;
            let col = sqr_idx % 3 * 3 + i % 3;

            if let Some(n) = self.nums.get(row, col) {
                sqr_list.push(*n);
            }
        }
        sqr_list
    }

    /// The set of numbers in the row with the given index.
    fn row_set(&self, row_idx: usize) -> NineSet {
        if row_idx > 8 {
            panic!("attempted to get row {}", row_idx)
        }

        NineSet::from(self.row_list(row_idx))
    }

    /// The set of numbers in the column with the given index.
    fn col_set(&self, col_idx: usize) -> NineSet {
        if col_idx > 8 {
            panic!("attempted to get column {}", col_idx)
        }

        NineSet::from(self.col_list(col_idx))
    }

    /// The set of numbers in the 3x3 square at the given index. Indices are in the
    /// range [0,9), begin in the upper left hand corner of the puzzle,
    /// and proceed left to right, top to bottom.
    fn sqr_set(&self, sqr_idx: usize) -> NineSet {
        assert!(sqr_idx < 9);
        NineSet::from(self.sqr_list(sqr_idx))
    }
}

impl fmt::Debug for SudokuPuzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.nums)
    }
}
