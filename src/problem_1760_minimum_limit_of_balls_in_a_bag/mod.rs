pub mod binary_search;

pub trait Solution {
    fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[9] as &[_], 2), 3), ((&[2, 4, 8, 2], 4), 2)];

        for ((nums, max_operations), expected) in test_cases {
            assert_eq!(S::minimum_size(nums.to_vec(), max_operations), expected);
        }
    }
}
