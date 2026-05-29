pub mod iterative;

pub trait Solution {
    fn alternating_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 5, 7] as &[_], -4), (&[100], 100)];

        for (nums, expected) in test_cases {
            assert_eq!(S::alternating_sum(nums.to_vec()), expected);
        }
    }
}
