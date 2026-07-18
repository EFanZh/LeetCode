pub mod iterative;

pub trait Solution {
    fn count_opposite_parity(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 4] as &[_], &[2, 1, 1, 0] as &[_]), (&[1], &[0])];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_opposite_parity(nums.to_vec()), expected);
        }
    }
}
