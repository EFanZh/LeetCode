pub mod dynamic_programming;
pub mod dynamic_programming_short;

pub trait Solution {
    fn nth_ugly_number(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 8),
            (8, 9),
            (9, 10),
            (10, 12),
            (11, 15),
            (12, 16),
            (13, 18),
            (14, 20),
            (15, 24),
            (16, 25),
            (17, 27),
            (18, 30),
            (19, 32),
            (20, 36),
            (21, 40),
            (22, 45),
            (23, 48),
            (24, 50),
            (25, 54),
            (26, 60),
            (27, 64),
            (28, 72),
            (29, 75),
            (30, 80),
            (31, 81),
            (32, 90),
            (33, 96),
            (34, 100),
            (35, 108),
            (36, 120),
            (37, 125),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::nth_ugly_number(n), expected);
        }
    }
}
