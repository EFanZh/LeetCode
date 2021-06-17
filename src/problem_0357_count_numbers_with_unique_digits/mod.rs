pub mod mathematical;
pub mod precomputed;

pub trait Solution {
    fn count_numbers_with_unique_digits(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 1),
            (1, 10),
            (2, 91),
            (3, 739),
            (4, 5275),
            (5, 32491),
            (6, 168_571),
            (7, 712_891),
            (8, 2_345_851),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::count_numbers_with_unique_digits(n), expected);
        }
    }
}
