pub mod hash_map;

pub trait Solution {
    fn most_frequent_even(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 1, 2, 2, 4, 4, 1] as &[_], 2),
            (&[4, 4, 4, 9, 2, 4], 4),
            (&[29, 47, 21, 41, 13, 37, 25, 7], -1),
            (
                &[8154, 9139, 8194, 3346, 5450, 9190, 133, 8239, 4606, 8671, 8412, 6290],
                3346,
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::most_frequent_even(nums.to_vec()), expected);
        }
    }
}
