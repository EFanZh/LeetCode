pub mod boyer_moore_majority_vote;

pub trait Solution {
    fn majority_element(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 2, 3] as &[_], &[3] as &[_]), (&[1], &[1]), (&[1, 2], &[1, 2])];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::majority_element(nums.iter().copied().collect()), expected);
        }
    }
}
