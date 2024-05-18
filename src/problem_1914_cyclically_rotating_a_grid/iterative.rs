pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;
use std::num::NonZeroUsize;

impl Solution {
    fn gcd(mut x: usize, mut y: NonZeroUsize) -> NonZeroUsize {
        while let Some(z) = NonZeroUsize::new(x % y) {
            x = y.get();
            y = z;
        }

        y
    }

    fn rotate_left<C, T>(
        length: NonZeroUsize,
        k: usize,
        container: &mut C,
        mut get_mut: impl FnMut(&mut C, usize) -> &mut T,
    ) where
        C: ?Sized,
        T: Clone,
    {
        if let Some(k) = NonZeroUsize::new(k % length) {
            let group_length = Self::gcd(length.get(), k);

            for start in 0..group_length.get() {
                let mut value = get_mut(container, start).clone();
                let mut index = start + length.get() - k.get();

                loop {
                    mem::swap(&mut value, get_mut(container, index));

                    if index < k.get() {
                        index += length.get();
                    }

                    index -= k.get();

                    if index == start {
                        break;
                    }
                }

                *get_mut(container, index) = value;
            }
        }
    }

    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let k = k as u32 as usize;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut length = (rows + columns + 2) * 2;
        let mut max_row = rows;
        let mut max_column = columns;
        let mut middle = rows + columns + 2;
        let mut layer_max_column = columns + 1;

        for layer in 0..rows.min(columns) / 2 {
            length -= 8;
            max_row -= 1;
            max_column -= 1;
            middle -= 4;
            layer_max_column -= 2;

            Self::rotate_left(NonZeroUsize::new(length).unwrap(), k, grid.as_mut_slice(), |grid, i| {
                #[allow(clippy::option_if_let_else)] // Expected.
                let (y, x) = if let Some(offset) = i.checked_sub(middle) {
                    if let Some(offset) = offset.checked_sub(layer_max_column) {
                        (max_row - offset, layer)
                    } else {
                        (max_row, max_column - offset)
                    }
                } else if let Some(offset) = i.checked_sub(layer_max_column) {
                    (layer + offset, max_column)
                } else {
                    (layer, layer + i)
                };

                &mut grid[y][x]
            });
        }

        grid
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        Self::rotate_grid(grid, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
