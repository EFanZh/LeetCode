pub mod iterative;

pub trait Solution {
    fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 3, 5, 1, 3] as &[_], 3), &[true, true, true, false, true] as &[_]),
            ((&[4, 2, 1, 1, 2], 1), &[true, false, false, false, false]),
            ((&[12, 1, 12], 10), &[true, false, true]),
        ];

        for ((candies, extra_candies), expected) in test_cases {
            assert_eq!(S::kids_with_candies(candies.to_vec(), extra_candies), expected);
        }
    }
}
