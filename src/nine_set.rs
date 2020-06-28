use std::fmt::Debug;

/// A Set collection that can hold numbers in the range [1,9].
#[derive(PartialEq, Clone, Copy)]
pub struct NineSet {
    contents: [bool; 9],
}

impl NineSet {
    /// Create and return an empty NineSet
    pub fn empty() -> Self {
        let contents = [false; 9];
        NineSet { contents }
    }

    /// Add n to this NineSet. n must be in the range [1,9]
    ///
    /// ```
    /// # use sudoku::nine_set::NineSet;
    /// let mut set = NineSet::empty();
    /// set.add(5);
    /// assert!(set.contains(5));
    /// ```
    pub fn add(&mut self, n: u8) {
        assert!(n >= 1);
        assert!(n <= 9);
        self.contents[(n - 1) as usize] = true;
    }

    /// Indicates whether this NineSet contains n.
    ///
    /// ```
    /// # use sudoku::nine_set::NineSet;
    /// let mut set = NineSet::empty();
    /// set.add(5);
    /// assert!(set.contains(5));
    /// assert!(!set.contains(6));
    /// ```
    pub fn contains(&self, n: u8) -> bool {
        n >= 1 && n <= 9 && self.contents[(n - 1) as usize]
    }

    /// Returns the number of unique numbers contained in this set.
    ///
    /// ```
    /// # use sudoku::nine_set::NineSet;
    /// let mut set = NineSet::empty();
    /// set.add(5);
    /// set.add(7);
    /// assert!(2, set.size());
    /// ```
    pub fn size(&self) -> usize {
        self.contents.iter().filter(|x| **x).count()
    }

    /// Creates and returns a Vec containing the numbers contained in this set.
    ///
    /// ```
    /// # use sudoku::nine_set::NineSet;
    /// let mut set = NineSet::empty();
    /// set.add(1);
    /// set.add(9);
    /// assert_eq!(vec![1, 9], set.to_vec());
    /// ```
    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        for n in 1..=9 {
            if self.contains(n) {
                vec.push(n as u8);
            }
        }
        vec
    }

    /// Creates and returns a set that contains all and only the numbers
    /// in the range [1, 9] that are not members of this set.
    ///
    ///
    /// ```
    /// # use sudoku::nine_set::NineSet;
    /// let mut set = NineSet::empty();
    /// set.add(1);
    /// set.add(9);
    /// assert_eq!(vec![2, 3, 4, 5, 6, 7, 8], set.complement().to_vec());
    /// ```
    pub fn complement(&self) -> Self {
        let mut contents = [true; 9];
        for i in 0..contents.len() {
            contents[i] = !self.contents[i];
        }
        NineSet { contents }
    }
}

impl From<Vec<u8>> for NineSet {
    fn from(nums: Vec<u8>) -> Self {
        let mut set = NineSet::empty();
        for num in nums {
            if num > 9 {
                panic!("Cannot add {} to NineSet", num)
            }
            set.add(num)
        }
        set
    }
}

impl Debug for NineSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NineSet:")?;
        f.debug_list().entries(self.to_vec().iter()).finish()
    }
}

/// Creates and returns a set containing all the numbers contained
/// by at least one of the sets in the provided list of sets.
///
/// ```
/// # use sudoku::nine_set::{NineSet, union};
/// let mut set1 = NineSet::empty();
/// let mut set2 = NineSet::empty();
/// set1.add(1);
/// set1.add(2);
/// set2.add(2);
/// set2.add(3);
/// let union = union(vec![set1, set2]).to_vec()
/// assert_eq!(vec![1, 2, 3], union);
/// ```
pub fn union(sets: Vec<NineSet>) -> NineSet {
    let mut union = NineSet::empty();
    for n in 1..=9 {
        if sets.iter().any(|s| s.contains(n)) {
            union.add(n);
        }
    }
    union
}
