pub mod backtracking;

pub trait Solution {
    fn total_n_queens(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A000170.
        let test_cases = [
            (0, 1),
            (1, 1),
            (2, 0),
            (3, 0),
            (4, 2),
            (5, 10),
            (6, 4),
            (7, 40),
            (8, 92),
            (9, 352),
            (10, 724),
            (11, 2680),
            (12, 14200),
            // (13, 73712),
            // (14, 365_596),
            // (15, 2_279_184),
            // (16, 14_772_512),
            // (17, 95_815_104),
            // (18, 666_090_624),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::total_n_queens(n), expected);
        }
    }
}
