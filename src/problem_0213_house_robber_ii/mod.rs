pub mod dynamic_programming;

pub trait Solution {
    fn rob(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 2] as &[_], 3), (&[1, 2, 3, 1], 4), (&[0], 0), (&[2, 3], 3)];

        for (nums, expected) in test_cases {
            assert_eq!(S::rob(nums.to_vec()), expected);
        }
    }
}
