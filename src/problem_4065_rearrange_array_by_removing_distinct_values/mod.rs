pub mod iterative;

pub trait Solution {
    fn rearrange_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 1, 3, 2, 1, 3] as &[_], &[1, 2, 3, 1, 3, 3] as &[_]),
            (&[7, 7, 4, 4, 4], &[4, 7, 4, 7, 4]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::rearrange_array(nums.to_vec()), expected);
        }
    }
}
