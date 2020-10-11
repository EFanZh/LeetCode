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
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::nth_ugly_number(n), expected);
        }
    }
}
