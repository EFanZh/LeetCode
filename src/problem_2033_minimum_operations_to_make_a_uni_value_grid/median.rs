pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let x = NonZeroU32::new(x as _).unwrap();
        let mut values = Vec::with_capacity(grid.first().map_or(0, Vec::len) * grid.len());
        let remainder = grid[0][0] as u32 % x;

        for row in grid {
            for value in row {
                let value = value as u32;

                if value % x == remainder {
                    let value = value / x;

                    values.push(value);
                } else {
                    return -1;
                }
            }
        }

        let n = values.len();
        let (left, &mut middle, right) = values.select_nth_unstable(n / 2);
        let mut result = 0;

        for &mut value in left {
            result += middle - value;
        }

        for &mut value in right {
            result += value - middle;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        Self::min_operations(grid, x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
