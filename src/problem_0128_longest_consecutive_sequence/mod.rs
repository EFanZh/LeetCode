pub mod hash_set;

pub trait Solution {
    fn longest_consecutive(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[100, 4, 200, 1, 3, 2] as &[_], 4)];

        for (nums, expected) in test_cases {
            assert_eq!(S::longest_consecutive(nums.to_vec()), expected);
        }
    }
}
