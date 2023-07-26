pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn max_absolute_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, -3, 2, 3, -4] as &[_], 5), (&[2, -5, 1, -4, 3, -2], 8)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_absolute_sum(nums.to_vec()), expected);
        }
    }
}
