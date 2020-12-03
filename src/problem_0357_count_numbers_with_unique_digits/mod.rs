pub mod compile_time_mathematical;
pub mod mathematical;

pub trait Solution {
    fn count_numbers_with_unique_digits(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(0, 1), (1, 10), (2, 91), (3, 739)];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::count_numbers_with_unique_digits(n), expected);
        }
    }
}
