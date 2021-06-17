pub mod stack;
pub mod stack_2;

pub trait Solution {
    fn find132pattern(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4] as &[_], false),
            (&[3, 1, 4, 2], true),
            (&[-1, 3, 2, 0], true),
            (&[40, 50, 25, 35, 15, 35, 20], true),
            (&[3, 5, 0, 3, 4], true),
            (&[2, 2, 1, 3], false),
            (&[26, 16, 16, 36, 36, 16, 16, 36, 36, 10, 10, 80, 80, 10, 10, 6], false),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::find132pattern(nums.to_vec()), expected);
        }
    }
}
