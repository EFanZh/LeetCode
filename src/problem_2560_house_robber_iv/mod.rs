pub mod binary_search;

pub trait Solution {
    fn min_capability(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 3, 5, 9] as &[_], 2), 5), ((&[2, 7, 9, 3, 1], 2), 2)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_capability(nums.to_vec(), k), expected);
        }
    }
}
