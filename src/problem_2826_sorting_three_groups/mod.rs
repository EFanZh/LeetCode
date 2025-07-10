pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn minimum_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 1, 3, 2, 1] as &[_], 3),
            (&[1, 3, 2, 1, 3, 3], 2),
            (&[2, 2, 2, 2, 3, 3], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_operations(nums.to_vec()), expected);
        }
    }
}
