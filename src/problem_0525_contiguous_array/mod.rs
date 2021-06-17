pub mod iterative;

pub trait Solution {
    fn find_max_length(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0, 1] as &[_], 2), (&[0, 1, 0], 2)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_max_length(nums.to_vec()), expected);
        }
    }
}
