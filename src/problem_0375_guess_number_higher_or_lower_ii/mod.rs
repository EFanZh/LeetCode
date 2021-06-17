pub mod dynamic_programming;

pub trait Solution {
    fn get_money_amount(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 0),
            (2, 1),
            (3, 2),
            (4, 4),
            (5, 6),
            (6, 8),
            (7, 10),
            (8, 12),
            (9, 14),
            (10, 16),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::get_money_amount(n), expected);
        }
    }
}
