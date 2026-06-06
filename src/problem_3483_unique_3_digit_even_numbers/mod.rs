pub mod iterative;

pub trait Solution {
    fn total_numbers(digits: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4] as &[_], 12),
            (&[0, 2, 2], 2),
            (&[6, 6, 6], 1),
            (&[1, 3, 5], 0),
        ];

        for (digits, expected) in test_cases {
            assert_eq!(S::total_numbers(digits.to_vec()), expected);
        }
    }
}
