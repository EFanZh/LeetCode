pub mod brute_force;

pub trait Solution {
    fn consecutive_numbers_sum(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A001227.
        let test_cases = [
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 1),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 1),
            (9, 3),
            (10, 2),
            (11, 2),
            (12, 2),
            (13, 2),
            (14, 2),
            (15, 4),
            (16, 1),
            (100, 3),
            (101, 2),
            (102, 4),
            (103, 2),
            (104, 2),
            (105, 8),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::consecutive_numbers_sum(n), expected);
        }
    }
}
