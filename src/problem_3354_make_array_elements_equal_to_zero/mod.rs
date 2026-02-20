pub mod iterative;

pub trait Solution {
    fn count_valid_selections(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 0, 2, 0, 3] as &[_], 2), (&[2, 3, 4, 0, 4, 1, 0], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_valid_selections(nums.to_vec()), expected);
        }
    }
}
