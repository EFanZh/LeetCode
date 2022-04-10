pub mod union_find;

pub trait Solution {
    fn largest_component_size(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 6, 15, 35] as &[_], 4),
            (&[20, 50, 9, 63], 2),
            (&[2, 3, 6, 7, 4, 12, 21, 39], 8),
            (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14], 11),
            (&[99, 68, 70, 77, 35, 52, 53, 25, 62], 8),
            (
                &[801, 839, 521, 999, 655, 16, 536, 84, 670, 983, 408, 346, 511, 638, 895],
                12,
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::largest_component_size(nums.to_vec()), expected);
        }
    }
}
