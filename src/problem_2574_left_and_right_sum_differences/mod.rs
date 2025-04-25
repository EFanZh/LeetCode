pub mod iterative;

pub trait Solution {
    fn left_right_difference(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[10, 4, 8, 3] as &[_], &[15, 1, 11, 22] as &[_]), (&[1], &[0])];

        for (nums, expected) in test_cases {
            assert_eq!(S::left_right_difference(nums.to_vec()), expected);
        }
    }
}
