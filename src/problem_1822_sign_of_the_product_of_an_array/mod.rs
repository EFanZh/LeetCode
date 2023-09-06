pub mod iterative;

pub trait Solution {
    fn array_sign(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[-1, -2, -3, -4, 3, 2, 1] as &[_], 1),
            (&[1, 5, 0, 2, -3], 0),
            (&[-1, 1, -1, 1, -1], -1),
            (&[1, -1, 0, 1, -1], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::array_sign(nums.to_vec()), expected);
        }
    }
}
