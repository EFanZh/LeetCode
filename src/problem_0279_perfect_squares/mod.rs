pub mod bfs;
pub mod bfs_2;

pub trait Solution {
    fn num_squares(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A002828.

        let test_cases = [
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 1),
            (5, 2),
            (6, 3),
            (7, 4),
            (8, 2),
            (9, 1),
            (10, 2),
            (11, 3),
            (12, 3),
            (13, 2),
            (14, 3),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::num_squares(n), expected);
        }
    }
}
