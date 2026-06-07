pub mod iterative;

pub trait Solution {
    fn min_moves(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 1, 3] as &[_], 3), (&[4, 4, 5], 2)];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_moves(nums.to_vec()), expected);
        }
    }
}
