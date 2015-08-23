
use math;
use num::Primitive;

pub trait Matrix<T: Primitive> : Sized
{
    /// Gets the identity matrix.
    // TODO: Make this an associated constant.
    fn identity() -> Self {
        Self::from_fn(math::util::kronecker_delta)
    }

    /// Creates a matrix from a function taking row and column numbers.
    fn from_fn<F>(f: F) -> Self
        where F: Fn(usize,usize) -> T;

    /// Gets the element at the given index.
    /// Panics on out of range.
    fn get(&self, row: usize, col: usize) -> T;

    /// Sets a value inside the matrix.
    /// Panics on out of range.
    fn set(&mut self, row: usize, col: usize, val: T);

    fn row<'a>(&'a self, num: usize) -> row::Row<'a,T>;
    fn col<'a>(&'a self, num: usize) -> column::Column<'a,T>;

    fn as_slice<'a>(&'a self) -> &'a [T];
    fn as_slice_mut<'a>(&'a mut self) -> &'a mut [T];
}

pub mod row {
    use math::Matrix;
    use std;

    /// A row in a matrix.
    pub struct Row<'a, T:'a> {
        /// The entire cell data.
        cells: &'a [T],
        /// The row number (zero based).
        row_num: usize,
        /// The number of columns in the row.
        width: usize,
    }

    impl<'a, T> Row<'a,T> {

        pub fn new(cells: &'a [T],
                   row_num: usize,
                   width: usize) -> Self {
            Row {
                cells: cells,
                row_num: row_num,
                width: width,
            }
        }
        
        pub fn iter(&'a self) -> Iter<'a,T> {
            Iter {
                row: self,
                cur_col: 0,
            }
        }
    }

    impl<'a,T> std::ops::Index<usize> for Row<'a,T>
    {
        type Output = T;

        fn index<'z>(&'z self, col_num: usize) -> &'z T {
            assert!(col_num < self.width, "out of bounds column number");
            &self.cells[self.row_num*self.width + col_num]
        }
    }

    pub struct Iter<'a,T:'a> {
        row: &'a Row<'a,T>,
        cur_col: usize,
    }

    impl<'a,T> Iterator for Iter<'a,T>
        where T: Clone
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            if self.cur_col < self.row.width {
                let elem = self.row[self.cur_col].clone();
                self.cur_col += 1;
                Some(elem)
            } else {
                None
            }
        }
    }
}

pub mod column {
    use std;

    pub struct Column<'a, T:'a> {
        /// The entire cell data.
        cells: &'a [T],
        /// The column number (zero based).
        col_num: usize,
        /// The number of columns in the matrix.
        width: usize,
        /// The number of rows in the matrix.
        height: usize,
    }

    impl<'a,T> Column<'a,T> {
        pub fn new(cells: &'a [T],
                   col_num: usize,
                   width: usize,
                   height: usize) -> Self {
            Column {
                cells: cells,
                col_num: col_num,
                width: width,
                height: height,
            }
        }

        pub fn iter(&'a self) -> Iter<'a,T> {
            Iter {
                col: self,
                cur_row: 0,
            }
        }
    }

    impl<'a,T> std::ops::Index<usize> for Column<'a,T>
    {
        type Output = T;

        fn index<'z>(&'z self, row_num: usize) -> &'z T {
            assert!(row_num < self.height, "out of bounds row number");
            &self.cells[row_num*self.width + self.col_num]
        }
    }

    pub struct Iter<'a,T:'a> {
        col: &'a Column<'a,T>,
        cur_row: usize,
    }

    impl<'a,T> Iterator for Iter<'a,T>
        where T: Clone
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            if self.cur_row < self.col.height {
                let elem = self.col[self.cur_row].clone();
                self.cur_row += 1;
                Some(elem)
            } else {
                None
            }
        }
    }
}
