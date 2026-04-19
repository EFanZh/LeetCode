pub mod iterative;

pub trait Solution {
    fn smallest_index(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 2] as &[_], 2), (&[1, 10, 11], 1), (&[1, 2, 3], -1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::smallest_index(nums.to_vec()), expected);
        }
    }
}
