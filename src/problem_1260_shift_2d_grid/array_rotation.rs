pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;
use std::num::NonZeroUsize;

impl Solution {
    fn gcd(mut x: usize, mut y: usize) -> usize {
        while y != 0 {
            let next_y = x % y;

            x = y;
            y = next_y;
        }

        x
    }

    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let rows = grid.len();
        let columns = NonZeroUsize::new(grid.first().map_or(0, Vec::len)).unwrap();
        let length = columns.get() * rows;
        let k = (k as usize) % length;

        for i in 0..Self::gcd(length, k) {
            let mut next = i;
            let mut value = grid[i / columns][i % columns];

            loop {
                next += k;
                next = next.checked_sub(length).unwrap_or(next);

                if next == i {
                    break;
                }

                mem::swap(&mut value, &mut grid[next / columns][next % columns]);
            }

            grid[i / columns][i % columns] = value;
        }

        grid
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        Self::shift_grid(grid, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
