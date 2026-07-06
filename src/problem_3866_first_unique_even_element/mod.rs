pub mod iterative;

pub trait Solution {
    fn first_unique_even(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 4, 2, 5, 4, 6] as &[_], 2),
            (&[4, 4], -1),
            (&[3, 4, 2, 5, 4, 6], 2),
            (&[2], 2),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::first_unique_even(nums.to_vec()), expected);
        }
    }
}
