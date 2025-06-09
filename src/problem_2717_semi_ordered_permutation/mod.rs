pub mod greedy;

pub trait Solution {
    fn semi_ordered_permutation(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 1, 4, 3] as &[_], 2), (&[2, 4, 1, 3], 3), (&[1, 3, 4, 2, 5], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::semi_ordered_permutation(nums.to_vec()), expected);
        }
    }
}
