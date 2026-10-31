pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut grid = grid;
        let k = NonZero::<u64>::new(u64::from(k.cast_unsigned())).unwrap();
        let mut result = 0;
        let mut stack = Vec::new();
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);

        for y in 0..rows {
            for x in 0..columns {
                if let Some(state) = grid
                    .get_mut(y)
                    .and_then(|row| row.get_mut(x))
                    .filter(|state| **state != 0)
                {
                    let mut sum = u64::from(state.cast_unsigned());

                    *state = 0;

                    let mut node = (y, x);

                    loop {
                        for neighbor in [
                            (node.0.wrapping_sub(1), node.1),
                            (node.0, node.1.wrapping_sub(1)),
                            (node.0, node.1 + 1),
                            (node.0 + 1, node.1),
                        ] {
                            if let Some(state) = grid
                                .get_mut(neighbor.0)
                                .and_then(|row| row.get_mut(neighbor.1))
                                .filter(|state| **state != 0)
                            {
                                sum += u64::from(state.cast_unsigned());
                                *state = 0;
                                stack.push((neighbor.0 as u16, neighbor.1 as u16));
                            }
                        }

                        if let Some(neighbor) = stack.pop() {
                            node = (usize::from(neighbor.0), usize::from(neighbor.1));
                        } else {
                            break;
                        }
                    }

                    result += i32::from(sum % k == 0);
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::count_islands(grid, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
