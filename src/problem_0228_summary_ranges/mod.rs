pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 1, 2, 4, 5, 7] as &[_], &["0->2", "4->5", "7"] as &[_]),
            (&[0, 2, 3, 4, 6, 8, 9], &["0", "2->4", "6", "8->9"]),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::summary_ranges(nums.to_vec()), expected);
        }
    }
}
