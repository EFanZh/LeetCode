pub mod iterative;

pub trait Solution {
    fn minimum_average_difference(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 5, 3, 9, 5, 3] as &[_], 3), (&[0], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_average_difference(nums.to_vec()), expected);
        }
    }
}
