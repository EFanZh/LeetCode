pub mod binary_search;
pub mod binary_search_fast;

pub trait Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    pub fn run_tests<S: super::Solution>() {
        let test_cases = vec![
            ((vec![1, 3, 5, 6], 5), 2),
            ((vec![1, 3, 5, 6], 2), 1),
            ((vec![1, 3, 5, 6], 7), 4),
            ((vec![1, 3, 5, 6], 0), 0),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::search_insert(nums, target), expected);
        }
    }
}
