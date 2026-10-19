pub mod iterative;

pub trait Solution {
    fn smallest_value(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // <https://oeis.org/A029908>.
        let test_cases = [
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 5),
            (7, 7),
            (8, 5),
            (9, 5),
            (10, 7),
            (11, 11),
            (12, 7),
            (13, 13),
            (14, 5),
            (15, 5),
            (16, 5),
            (17, 17),
            (18, 5),
            (19, 19),
            (20, 5),
            (21, 7),
            (22, 13),
            (23, 23),
            (24, 5),
            (25, 7),
            (26, 5),
            (27, 5),
            (28, 11),
            (29, 29),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::smallest_value(n), expected);
        }
    }
}
