pub mod iterative;

pub trait Solution {
    fn array_nesting(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 4, 0, 3, 1, 6, 2] as &[_], 4), (&[0, 1, 2], 1)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::array_nesting(nums.to_vec()), expected);
        }
    }
}
