pub mod greedy;

pub trait Solution {
    fn min_patches(nums: Vec<i32>, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 3] as &[_], 6), 1), ((&[1, 5, 10], 20), 2), ((&[1, 2, 2], 5), 0)];

        for ((nums, n), expected) in test_cases.iter().copied() {
            assert_eq!(S::min_patches(nums.to_vec(), n), expected);
        }
    }
}
