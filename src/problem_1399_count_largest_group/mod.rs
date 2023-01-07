pub mod brute_force;

pub trait Solution {
    fn count_largest_group(n: i32) -> i32;
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
            (7, 7),
            (8, 8),
            (9, 9),
            (10, 1),
            (11, 2),
            (12, 3),
            (13, 4),
            (14, 5),
            (15, 6),
            (16, 7),
            (17, 8),
            (18, 9),
            (19, 9),
            (20, 1),
            (21, 2),
            (22, 3),
            (23, 4),
            (24, 5),
            (25, 6),
            (26, 7),
            (27, 8),
            (28, 8),
            (29, 8),
            (30, 1),
            (31, 2),
            (32, 3),
            (10000, 1),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::count_largest_group(n), expected);
        }
    }
}
