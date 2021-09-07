pub mod iterative;

pub trait Solution {
    fn check_possibility(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 2, 3] as &[_], true),
            (&[4, 2, 1], false),
            (&[3, 4, 2, 3], false),
            (&[1], true),
            (&[5, 7, 1, 8], true),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::check_possibility(nums.to_vec()), expected);
        }
    }
}
