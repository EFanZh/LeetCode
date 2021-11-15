pub mod iterative;

pub trait Solution {
    fn longest_mountain(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 1, 4, 7, 3, 2, 5] as &[_], 5),
            (&[2, 2, 2], 0),
            (&[8, 3, 7, 3, 4, 10, 1, 1, 2, 8], 4),
            (&[0, 0, 1, 0, 0, 1, 1, 1, 1, 1], 3),
            (&[0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0], 11),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::longest_mountain(arr.to_vec()), expected);
        }
    }
}
