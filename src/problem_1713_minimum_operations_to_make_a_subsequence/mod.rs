pub mod longest_increasing_subsequence;

pub trait Solution {
    fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 1, 3] as &[_], &[9, 4, 2, 3, 4] as &[_]), 2),
            ((&[6, 4, 8, 1, 3, 2], &[4, 7, 6, 2, 3, 8, 6, 1]), 3),
        ];

        for ((target, arr), expected) in test_cases {
            assert_eq!(S::min_operations(target.to_vec(), arr.to_vec()), expected);
        }
    }
}
