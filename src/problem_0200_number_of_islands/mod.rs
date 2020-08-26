pub mod bfs;
pub mod dfs;
pub mod recursive_dfs;

pub trait Solution {
    fn num_islands(grid: Vec<Vec<char>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    &['1', '1', '1', '1', '0'] as &[_],
                    &['1', '1', '0', '1', '0'],
                    &['1', '1', '0', '0', '0'],
                    &['0', '0', '0', '0', '0'],
                ] as &[&[_]],
                1,
            ),
            (
                &[
                    &['1', '1', '0', '0', '0'],
                    &['1', '1', '0', '0', '0'],
                    &['0', '0', '1', '0', '0'],
                    &['0', '0', '0', '1', '1'],
                ],
                3,
            ),
        ];

        for (grid, expected) in test_cases.iter().copied() {
            assert_eq!(S::num_islands(grid.iter().map(|row| row.to_vec()).collect()), expected);
        }
    }
}
