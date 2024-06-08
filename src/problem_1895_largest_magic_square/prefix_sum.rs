pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut sums = vec![(0, 0, 0, 0); columns * rows].into_boxed_slice();
        let mut index = 0;

        for (y, row) in grid.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                sums[columns * y + x] = (
                    if x == 0 { value } else { value + sums[index - 1].0 },
                    if y == 0 { value } else { value + sums[index - columns].1 },
                    if x == 0 || y == 0 {
                        value
                    } else {
                        value + sums[index - (columns + 1)].2
                    },
                    if x == columns - 1 || y == 0 {
                        value
                    } else {
                        value + sums[index - (columns - 1)].3
                    },
                );

                index += 1;
            }
        }

        for size in (2..=rows.min(columns)).rev() {
            let mut index_start = 0;

            for y in 0..=rows - size {
                for x in 0..=columns - size {
                    let index = index_start + x;
                    let expected = sums[index + size - 1].0 - if x == 0 { 0 } else { sums[index - 1].0 };

                    if (index + columns..index + columns * size)
                        .step_by(columns)
                        .all(|i| sums[i + size - 1].0 - if x == 0 { 0 } else { sums[i - 1].0 } == expected)
                        && (index..index + size).all(|i| {
                            sums[i + columns * (size - 1)].1
                                - sums.get(i.wrapping_sub(columns)).map_or(0, |sums| sums.1)
                                == expected
                        })
                        && sums[index + (columns + 1) * (size - 1)].2
                            - if x == 0 || y == 0 {
                                0
                            } else {
                                sums[index - (columns + 1)].2
                            }
                            == expected
                        && sums[index + columns * (size - 1)].3
                            - if x == columns - size || y == 0 {
                                0
                            } else {
                                sums[index - columns + size].3
                            }
                            == expected
                    {
                        return size as _;
                    }
                }

                index_start += columns;
            }
        }

        1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        Self::largest_magic_square(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
