pub mod iterative;

pub trait Solution {
    fn check(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 4, 5, 1, 2] as &[_], true),
            (&[2, 1, 3, 4], false),
            (&[1, 2, 3], true),
            (&[1, 1, 1], true),
            (&[2, 1], true),
            (&[8, 5, 4, 5, 1, 4, 5, 2, 2], false),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::check(nums.to_vec()), expected);
        }
    }
}
