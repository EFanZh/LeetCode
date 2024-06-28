pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut sums = vec![(0_u32, 0_u32); columns * rows].into_boxed_slice();

        for (y, row) in grid.into_iter().enumerate() {
            for (x, value) in row.into_iter().enumerate() {
                let index = columns * y + x;
                let value = value as u32;

                sums[index] = (
                    if y == 0 || x == 0 {
                        value
                    } else {
                        sums[index - (columns + 1)].0 + value
                    },
                    if y == 0 || x >= columns - 1 {
                        value
                    } else {
                        sums[index - (columns - 1)].1 + value
                    },
                );
            }
        }

        let mut result = Vec::<i32>::new();

        let mut add_result = |value: u32| {
            if let Err(i) = result.binary_search_by(|&x| value.cmp(&(x as u32))) {
                if i < 3 {
                    if result.len() == 3 {
                        result.pop();
                    }

                    result.insert(i, value as _);
                }
            }
        };

        let get_sum = |y: usize, x: usize| sums[columns * y + x];

        for y in 0..rows {
            for x in 0..columns {
                let bottom_sum = get_sum(y, x);

                add_result(bottom_sum.0 - if y == 0 || x == 0 { 0 } else { get_sum(y - 1, x - 1).0 });

                let max_size = x.min(columns - 1 - x).min(y / 2);

                for size in 1..=max_size {
                    let top = (y - size * 2, x);
                    let left = (y - size, x - size);
                    let right = (y - size, x + size);

                    let sum_1 = get_sum(right.0 - 1, right.1 - 1).0
                        - if top.0 == 0 { 0 } else { get_sum(top.0 - 1, top.1 - 1).0 };

                    let sum_2 = get_sum(y - 1, x + 1).1
                        - if right.1 >= columns - 1 {
                            0
                        } else {
                            get_sum(right.0 - 1, right.1 + 1).1
                        };

                    let sum_3 = bottom_sum.0 - get_sum(left.0, left.1).0;

                    let sum_4 = get_sum(left.0, left.1).1 - get_sum(top.0, top.1).1;

                    add_result(sum_1 + sum_2 + sum_3 + sum_4);
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        Self::get_biggest_three(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
