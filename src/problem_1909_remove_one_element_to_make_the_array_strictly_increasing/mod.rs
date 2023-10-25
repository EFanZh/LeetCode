pub mod iterative;

pub trait Solution {
    fn can_be_increasing(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 10, 5, 7] as &[_], true),
            (&[2, 3, 1, 2], false),
            (&[1, 1, 1], false),
            (&[105, 924, 32, 968], true),
            (&[1, 2, 3], true),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::can_be_increasing(nums.to_vec()), expected);
        }
    }
}
