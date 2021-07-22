pub mod bfs;

pub trait Solution {
    fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    "0010000100000",
                    "0000000111000",
                    "0110100000000",
                    "0100110010100",
                    "0100110011100",
                    "0000000000100",
                    "0000000111000",
                    "0000000110000",
                ] as &[_],
                6,
            ),
            (&["00000000"], 0),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(
                S::max_area_of_island(
                    grid.iter()
                        .map(|s| s.bytes().map(|c| i32::from(c - b'0')).collect())
                        .collect()
                ),
                expected
            );
        }
    }
}
