pub mod iterative;

pub trait Solution {
    fn valid_mountain_array(arr: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 1] as &[_], false),
            (&[3, 5, 5], false),
            (&[0, 3, 2, 1], true),
            (&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], false),
            (&[2], false),
            (&[0, 1, 2, 1, 2], false),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::valid_mountain_array(arr.to_vec()), expected);
        }
    }
}
