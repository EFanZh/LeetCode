pub mod binary_search;

pub trait Solution {
    fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 10, 100] as &[_], &[1000] as &[_]), 3),
            ((&[1, 2, 3], &[4, 4, 4]), 0),
        ];

        for ((arr1, arr2), expected) in test_cases {
            assert_eq!(S::longest_common_prefix(arr1.to_vec(), arr2.to_vec()), expected);
        }
    }
}
