pub mod reduce_xor;

pub trait Solution {
    fn single_number(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 2, 1] as &[_], 1), (&[4, 1, 2, 1, 2], 4)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::single_number(nums.to_vec()), expected);
        }
    }
}
