pub mod iterative;
pub mod iterative_2;
pub mod iterative_3;
pub mod tail_recursive;

pub trait Solution {
    fn find_max_consecutive_ones(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 0, 1, 1, 1] as &[_], 3), (&[0, 0, 1, 1, 1, 0, 0], 3)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_max_consecutive_ones(nums.to_vec()), expected);
        }
    }
}
