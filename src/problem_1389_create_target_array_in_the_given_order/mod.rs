pub mod brute_force;
pub mod merge_sort;

pub trait Solution {
    fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[0, 1, 2, 3, 4] as &[_], &[0, 1, 2, 2, 1] as &[_]),
                &[0, 4, 1, 3, 2] as &[_],
            ),
            ((&[1, 2, 3, 4, 0], &[0, 1, 2, 3, 0]), &[0, 1, 2, 3, 4]),
            ((&[1], &[0]), &[1]),
            ((&[4, 2, 4, 3, 2], &[0, 0, 1, 3, 1]), &[2, 2, 4, 4, 3]),
            (
                (
                    &[18, 4, 15, 10, 12, 15, 17, 18, 7, 16, 8, 11, 11, 7, 13],
                    &[0, 1, 0, 3, 2, 5, 6, 0, 2, 7, 5, 7, 6, 5, 7],
                ),
                &[18, 15, 7, 18, 12, 7, 8, 13, 11, 4, 11, 10, 16, 15, 17],
            ),
        ];

        for ((nums, index), expected) in test_cases {
            assert_eq!(S::create_target_array(nums.to_vec(), index.to_vec()), expected);
        }
    }
}
