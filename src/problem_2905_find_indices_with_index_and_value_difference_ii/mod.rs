pub mod iterative;

pub trait Solution {
    fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 1, 4, 1] as &[_], 2, 4), true),
            ((&[2, 1], 0, 0), true),
            ((&[1, 2, 3], 2, 4), false),
            ((&[0], 0, 0), true),
            ((&[0], 100, 50), false),
        ];

        for ((nums, index_difference, value_difference), expected) in test_cases {
            let result =
                <[_; 2]>::try_from(S::find_indices(nums.to_vec(), index_difference, value_difference)).unwrap();

            if expected {
                assert!(result[1] - result[0] >= index_difference);

                assert!(
                    nums[result[0] as u32 as usize].abs_diff(nums[result[1] as u32 as usize])
                        >= value_difference as u32
                );
            } else {
                assert_eq!(result, [-1, -1]);
            }
        }
    }
}
