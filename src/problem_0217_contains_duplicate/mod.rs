pub mod hash_set;

pub trait Solution {
    fn contains_duplicate(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 1] as &[_], true),
            (&[1, 2, 3, 4], false),
            (&[1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::contains_duplicate(nums.to_vec()), expected);
        }
    }
}
