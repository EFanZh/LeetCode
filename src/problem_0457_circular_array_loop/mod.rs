pub mod iterative;

pub trait Solution {
    fn circular_array_loop(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, -1, 1, 2, 2] as &[_], true),
            (&[-1, 2], false),
            (&[-2, 1, -1, -2, -2], false),
            (&[-1, -2, -3, -4, -5], false),
            (&[-1, -1, -1], true),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::circular_array_loop(nums.to_vec()), expected);
        }
    }
}
