pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn zip_3<I>(values_0: I, values_1: I, values_2: I) -> impl Iterator<Item = (I::Item, I::Item, I::Item)>
    where
        I: IntoIterator,
    {
        values_0
            .into_iter()
            .zip(values_1)
            .zip(values_2)
            .map(|((x, y), z)| (x, y, z))
    }

    fn windows_3<T>(values: &[T]) -> impl Iterator<Item = (&T, &T, &T)> {
        Self::zip_3(values, &values[1..], &values[2..])
    }

    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        if grid.len() >= 3 && grid.first().map_or(0, Vec::len) >= 3 {
            for (row_0, row_1, row_2) in Self::windows_3(&grid) {
                for square in Self::zip_3(Self::windows_3(row_0), Self::windows_3(row_1), Self::windows_3(row_2)) {
                    if matches!(
                        square,
                        ((2, 7, 6), (9, 5, 1), (4, 3, 8))
                            | ((2, 9, 4), (7, 5, 3), (6, 1, 8))
                            | ((4, 3, 8), (9, 5, 1), (2, 7, 6))
                            | ((4, 9, 2), (3, 5, 7), (8, 1, 6))
                            | ((6, 1, 8), (7, 5, 3), (2, 9, 4))
                            | ((6, 7, 2), (1, 5, 9), (8, 3, 4))
                            | ((8, 1, 6), (3, 5, 7), (4, 9, 2))
                            | ((8, 3, 4), (1, 5, 9), (6, 7, 2))
                    ) {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        Self::num_magic_squares_inside(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
