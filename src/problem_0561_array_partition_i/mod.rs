pub mod sorting;

pub trait Solution {
    fn array_pair_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 4, 3, 2] as &[_], 4), (&[6, 2, 6, 5, 1, 2], 9)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::array_pair_sum(nums.to_vec()), expected);
        }
    }
}
