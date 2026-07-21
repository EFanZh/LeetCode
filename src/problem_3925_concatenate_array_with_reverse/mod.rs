pub mod iterative;

pub trait Solution {
    fn concat_with_reverse(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3] as &[_], &[1, 2, 3, 3, 2, 1] as &[_]), (&[1], &[1, 1])];

        for (nums, expected) in test_cases {
            assert_eq!(S::concat_with_reverse(nums.to_vec()), expected);
        }
    }
}
