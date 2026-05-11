pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let x = x as _;
        let y = y as usize;
        let k = k as _;

        (y..y + k).for_each(|column| {
            let mut iter = grid.iter_mut().skip(x).take(k);

            while let Some(left) = iter.next() {
                if let Some(right) = iter.next_back() {
                    mem::swap(&mut left[column], &mut right[column]);
                } else {
                    break;
                }
            }
        });

        grid
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        Self::reverse_submatrix(grid, x, y, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
