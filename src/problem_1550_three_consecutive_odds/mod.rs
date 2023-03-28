pub mod iterative;

pub trait Solution {
    fn three_consecutive_odds(arr: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 6, 4, 1] as &[_], false),
            (&[1, 2, 34, 3, 4, 5, 7, 23, 12], true),
            (&[1, 3, 2], false),
            (&[1, 3], false),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::three_consecutive_odds(arr.to_vec()), expected);
        }
    }
}
