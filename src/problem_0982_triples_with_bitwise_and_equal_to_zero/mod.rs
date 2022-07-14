pub mod iterative;

pub trait Solution {
    fn count_triplets(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 1, 3] as &[_], 12), (&[0, 0, 0], 27)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_triplets(nums.to_vec()), expected);
        }
    }
}
