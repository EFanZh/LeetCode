pub mod iterative;
pub mod iterative_2;
pub mod iterative_3;

pub trait Solution {
    fn is_ideal_permutation(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0] as &[_], true),
            (&[0, 1], true),
            (&[1, 0], true),
            (&[0, 1, 2], true),
            (&[0, 2, 1], true),
            (&[1, 0, 2], true),
            (&[1, 2, 0], false),
            (&[2, 0, 1], false),
            (&[2, 1, 0], false),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::is_ideal_permutation(nums.to_vec()), expected);
        }
    }
}
