use std::{
    fmt::{self, Debug},
    iter::IntoIterator,
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

use qvek::uuvec2;

// You should index into it in this order
//  for y in 0..y_size {
//  for x in 0..x_size {
// Currently, Rust cannot optimize the order automatically. TODO: MIR-OPT coherence transform

#[derive(Clone)]
pub struct DArray2D<T> {
    pub data: Box<[T]>,
    pub x_size: usize,
    pub y_size: usize,
}

impl<T> DArray2D<T> {
    pub fn new_filled_by_generator(
        x_size: usize,
        y_size: usize,
        generator: impl Fn() -> T,
    ) -> Self {
        assert!(
            x_size > 0 && y_size > 0,
            "Dimensions must be greater than zero"
        );
        let data: Vec<T> = (0..x_size * y_size).map(|_| generator()).collect();
        Self {
            data: data.into_boxed_slice(),
            x_size,
            y_size,
        }
    }

    /// Returns the index in the flat data array for given (x, y) coordinates.
    pub fn index_internal(&self, x: usize, y: usize) -> usize {
        // not worth keeping in release
        #[cfg(debug_assertions)] // lets not stress optimizer
        if !(x < self.x_size && y < self.y_size) {
            panic!("Index out of bounds x: {} y: {}\n", x, y);
        };
        // optimal for
        // for y in 0..y_size {
        //     for x in 0..x_size {
        // indexing
        y * self.x_size + x
    }

    /// Returns the dimensions of the array.
    pub fn dimensions(&self) -> (usize, usize) {
        (self.x_size, self.y_size)
    }

    /// Returns the dimensions of the array.
    pub fn size(&self) -> uuvec2 {
        uuvec2::new(self.x_size, self.y_size)
    }

    /// Returns an iterator over the array.
    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the array.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn get_ref(&self, x: usize, y: usize) -> &T {
        &self.data[self.index_internal(x, y)]
    }
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        let idx = self.index_internal(x, y);
        self.data[idx] = value;
    }
}

impl<T: Clone> DArray2D<T> {
    /// Creates a new 2D array with given dimensions, initialized with `Default` values.
    pub fn new(x_size: usize, y_size: usize) -> Self
    where
        T: Default + Clone,
    {
        Self::new_filled(x_size, y_size, T::default())
    }

    pub fn new_filled(x_size: usize, y_size: usize, value: T) -> Self {
        // woth keeping in release
        assert!(
            x_size > 0 && y_size > 0,
            "Dimensions must be greater than zero"
        );
        let data = vec![value; x_size * y_size];
        Self {
            data: data.into_boxed_slice(),
            x_size,
            y_size,
        }
    }

    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }

    pub fn copy_data_from(&mut self, other: &DArray2D<T>) {
        self.data = other.data.clone();
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[self.index_internal(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[self.index_internal(x, y)]
    }
}

impl<T: Clone> Index<(usize, usize)> for DArray2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index; // unpack the tuple
        let index_internal = self.index_internal(x, y);
        &self.data[index_internal]
    }
}

impl<T: Clone> Index<(i8, i8)> for DArray2D<T> {
    type Output = T;

    fn index(&self, index: (i8, i8)) -> &Self::Output {
        let (x, y) = index; // unpack the tuple
        let index_internal = self.index_internal(x as usize, y as usize);
        &self.data[index_internal]
    }
}

impl<T: Clone> Index<(i32, i32)> for DArray2D<T> {
    type Output = T;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        let (x, y) = index; // unpack the tuple
        let index_internal = self.index_internal(x as usize, y as usize);
        &self.data[index_internal]
    }
}

impl<T: Clone> Index<qvek::types::ivec2> for DArray2D<T> {
    type Output = T;

    fn index(&self, index: qvek::types::ivec2) -> &Self::Output {
        let (x, y) = (index.x, index.y); // unpack the tuple
        let index_internal = self.index_internal(x as usize, y as usize);
        &self.data[index_internal]
    }
}
impl<T: Clone> Index<qvek::types::iivec2> for DArray2D<T> {
    type Output = T;

    fn index(&self, index: qvek::types::iivec2) -> &Self::Output {
        let (x, y) = (index.x, index.y); // unpack the tuple
        let index_internal = self.index_internal(x as usize, y as usize);
        &self.data[index_internal]
    }
}
impl<T: Clone> IndexMut<qvek::types::iivec2> for DArray2D<T> {
    fn index_mut(&mut self, index: qvek::types::iivec2) -> &mut Self::Output {
        let (x, y) = (index.x, index.y); // unpack the tuple
        let index_internal = self.index_internal(x as usize, y as usize);
        &mut self.data[index_internal]
    }
}

impl<T: Clone> Index<qvek::types::uuvec2> for DArray2D<T> {
    type Output = T;

    fn index(&self, index: qvek::types::uuvec2) -> &Self::Output {
        let (x, y) = (index.x, index.y); // unpack the tuple
        let index_internal = self.index_internal(x as usize, y as usize);
        &self.data[index_internal]
    }
}
impl<T: Clone> IndexMut<qvek::types::uuvec2> for DArray2D<T> {
    fn index_mut(&mut self, index: qvek::types::uuvec2) -> &mut Self::Output {
        let (x, y) = (index.x, index.y); // unpack the tuple
        let index_internal = self.index_internal(x as usize, y as usize);
        &mut self.data[index_internal]
    }
}

impl<T: Clone> IndexMut<(usize, usize)> for DArray2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index; // unpack the tuple
        let index_internal = self.index_internal(x, y);
        &mut self.data[index_internal]
    }
}

impl<T: Default + Clone> Debug for DArray2D<T>
where
    T: Debug,
{
    // a lot more readable
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x_size, y_size) = self.dimensions();
        writeln!(f, "Array2D [{} x {}]:", x_size, y_size)?;
        for y in 0..y_size {
            write!(f, "[ ")?;
            for x in 0..x_size {
                write!(f, "{:?} ", self[(x, y)])?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl<T: Clone> IntoIterator for DArray2D<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_vec().into_iter()
    }
}

impl<'a, T: Clone> IntoIterator for &'a DArray2D<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Clone> IntoIterator for &'a mut DArray2D<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_and_access() {
        let mut array = DArray2D::new(2, 3);
        assert_eq!(array.dimensions(), (2, 3));

        array[(0, 0)] = 42;
        array[(1, 2)] = 99;

        assert_eq!(array[(0, 0)], 42);
        assert_eq!(array[(1, 2)], 99);
    }

    #[test]
    fn test_iteration() {
        let array: DArray2D<i32> = DArray2D::new(2, 2);
        assert_eq!(array.iter().count(), 4);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let array = DArray2D::new(2, 2);
        let _: i32 = array[(2, 2)]; // Should panic
    }
}
