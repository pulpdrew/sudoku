use std::fmt;

/// A 9x9 array of `Option<T>`'s
#[derive(Clone)]
pub struct NineByNine<T> {
    data: [Option<T>; 81],
}

impl<T> From<[Option<T>; 81]> for NineByNine<T> {
    /// Create and return a new NineByNine containing data, where data is a linear,
    /// row-major representation of the 2d array that will be created.
    fn from(data: [Option<T>; 81]) -> Self {
        Self { data }
    }
}

impl<T: Copy> NineByNine<T> {
    /// Create a new 9x9 that consists of only `None` elements.
    pub fn new() -> Self {
        Self { data: [None; 81] }
    }

    /// Indicates the number of None elements in this 9x9.
    pub fn count_nones(&self) -> usize {
        self.data.iter().filter(|x| x.is_none()).count()
    }

    /// Get the data element at (row, col).
    /// row and column must each be in the range [0, 9).
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        assert!(row < 9);
        assert!(col < 9);

        self.data[row * 9 + col].as_ref()
    }

    /// Set the data element at (row, col);
    /// row and column must each be in the range [0, 9).
    pub fn set(&mut self, row: usize, col: usize, val: Option<T>) {
        assert!(row < 9);
        assert!(col < 9);

        self.data[row * 9 + col] = val;
    }
}

impl<T: fmt::Debug + Copy> fmt::Debug for NineByNine<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let to_string = |n: Option<&T>| match n {
            Some(n) => format!("| {:?} ", n),
            None => String::from("|   "),
        };

        let mut separator = String::new();
        for row in 0..9 {
            let mut row_str = String::new();
            for col in 0..9 {
                row_str.push_str(&to_string(self.get(row, col)))
            }
            separator = std::iter::repeat('-').take(row_str.len() + 1).collect();
            write!(f, "{}\n{}|\n", separator, row_str).unwrap();
        }

        write!(f, "{}", separator)
    }
}
