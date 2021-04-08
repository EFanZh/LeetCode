pub mod backtracking;

pub trait Solution {
    fn makesquare(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 2, 2, 2] as &[_], true),
            (&[3, 3, 3, 3, 4], false),
            (&[], false),
            (&[5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3], true),
            (&[2, 2, 2, 2, 2, 6], false),
            (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 4, 3, 2, 1], false),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::makesquare(nums.to_vec()), expected);
        }
    }
}
