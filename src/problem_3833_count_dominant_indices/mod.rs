pub mod iterative;

pub trait Solution {
    fn dominant_indices(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 4, 3] as &[_], 2), (&[4, 1, 2], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::dominant_indices(nums.to_vec()), expected);
        }
    }
}
