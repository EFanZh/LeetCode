pub mod iterative;

pub trait Solution {
    fn find_lhs(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 3, 2, 2, 5, 2, 3, 7] as &[_], 5),
            (&[1, 2, 3, 4], 2),
            (&[1, 1, 1, 1], 0),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::find_lhs(nums.to_vec()), expected);
        }
    }
}
