pub mod memoized_dynamic_programming;
pub mod memoized_dynamic_programming_2;

pub trait Solution {
    fn min_days(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // <https://oeis.org/A056796>.
        let test_cases = [
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 4),
            (6, 3),
            (7, 4),
            (8, 4),
            (9, 3),
            (10, 4),
            (11, 5),
            (12, 4),
            (13, 5),
            (14, 5),
            (15, 5),
            (16, 5),
            (17, 6),
            (18, 4),
            (19, 5),
            (20, 5),
            (21, 5),
            (22, 6),
            (23, 7),
            (24, 5),
            (25, 6),
            (26, 6),
            (27, 4),
            (28, 5),
            (29, 6),
            (30, 5),
            (31, 6),
            (32, 6),
            (33, 6),
            (34, 7),
            (35, 8),
            (36, 5),
            (37, 6),
            (38, 6),
            (39, 6),
            (1_999_999_943, 36),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::min_days(n), expected);
        }
    }
}
