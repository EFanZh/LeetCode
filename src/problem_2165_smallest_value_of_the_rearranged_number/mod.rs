pub mod iterative;

pub trait Solution {
    fn smallest_number(num: i64) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (-7605, -7650),
            (-12, -21),
            (-11, -11),
            (-10, -10),
            (-9, -9),
            (-8, -8),
            (-1, -1),
            (0, 0),
            (1, 1),
            (2, 2),
            (8, 8),
            (9, 9),
            (10, 10),
            (11, 11),
            (12, 12),
            (19, 19),
            (20, 20),
            (310, 103),
        ];

        for (num, expected) in test_cases {
            assert_eq!(S::smallest_number(num), expected);
        }
    }
}
