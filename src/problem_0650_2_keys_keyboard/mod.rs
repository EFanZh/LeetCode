pub mod bfs;
pub mod prime_factorization;
pub trait Solution {
    fn min_steps(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A001414.
        let test_cases = [
            (1, 0),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 5),
            (7, 7),
            (8, 6),
            (9, 6),
            (10, 7),
            (11, 11),
            (12, 7),
            (13, 13),
            (14, 9),
            (15, 8),
            (16, 8),
            (17, 17),
            (311, 311),
            (727, 727),
            (983, 983),
            (997, 997),
            (1000, 21),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::min_steps(n), expected);
        }
    }
}
