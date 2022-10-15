pub mod iterative;

pub trait Solution {
    fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 2, 2, 2, 5, 5, 5, 8] as &[_], 3, 4), 3),
            ((&[11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5), 6),
        ];

        for ((arr, k, threshold), expected) in test_cases {
            assert_eq!(S::num_of_subarrays(arr.to_vec(), k, threshold), expected);
        }
    }
}
