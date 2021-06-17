pub mod clever;

pub trait Solution {
    fn plus_one(digits: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3] as &[_], &[1, 2, 4] as &[_]),
            (&[4, 3, 2, 1], &[4, 3, 2, 2]),
            (&[9, 9, 9, 9], &[1, 0, 0, 0, 0]),
        ];

        for (digits, expected) in test_cases {
            assert_eq!(S::plus_one(digits.to_vec()), expected);
        }
    }
}
