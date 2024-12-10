pub mod bit_masks;

pub trait Solution {
    fn minimum_operations(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 5, 0, 3, 5] as &[_], 3), (&[0], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_operations(nums.to_vec()), expected);
        }
    }
}
