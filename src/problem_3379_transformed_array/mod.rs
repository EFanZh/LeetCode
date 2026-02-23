pub mod iterative;

pub trait Solution {
    fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, -2, 1, 1] as &[_], &[1, 1, 1, 3] as &[_]),
            (&[-1, 4, -1], &[-1, -1, 4]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::construct_transformed_array(nums.to_vec()), expected);
        }
    }
}
