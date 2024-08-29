pub mod longest_increasing_subsequence;
pub mod longest_increasing_subsequence_2;

pub trait Solution {
    fn k_increasing(arr: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 4, 3, 2, 1] as &[_], 1), 4),
            ((&[4, 1, 5, 2, 6, 2], 2), 0),
            ((&[4, 1, 5, 2, 6, 2], 3), 2),
            ((&[12, 6, 12, 6, 14, 2, 13, 17, 3, 8, 11, 7, 4, 11, 18, 8, 8, 3], 1), 12),
        ];

        for ((arr, k), expected) in test_cases {
            assert_eq!(S::k_increasing(arr.to_vec(), k), expected);
        }
    }
}
