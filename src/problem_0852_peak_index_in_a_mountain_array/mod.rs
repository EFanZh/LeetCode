pub mod binary_search;
pub mod iterative;

pub trait Solution {
    fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 1, 0] as &[_], 1),
            (&[0, 2, 1, 0], 1),
            (&[0, 10, 5, 2], 1),
            (&[3, 4, 5, 1], 2),
            (&[24, 69, 100, 99, 79, 78, 67, 36, 26, 19], 2),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::peak_index_in_mountain_array(arr.to_vec()), expected);
        }
    }
}
