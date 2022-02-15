pub mod sliding_window;

pub trait Solution {
    fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 0, 1, 0, 1] as &[_], 2), 4),
            ((&[0, 0, 0, 0, 0], 0), 15),
            ((&[0, 0, 0, 0, 0, 0, 1, 0, 0, 0], 0), 27),
            ((&[0, 0, 0, 0, 1], 2), 0),
        ];

        for ((nums, goal), expected) in test_cases {
            assert_eq!(S::num_subarrays_with_sum(nums.to_vec(), goal), expected);
        }
    }
}
