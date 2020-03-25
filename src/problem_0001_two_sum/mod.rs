pub mod index_map;
pub mod sort_then_bidirectional_search;
pub mod sort_then_bidirectional_search_2;

pub trait Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = vec![
            ((vec![2, 7, 11, 15], 9), vec![0, 1]),
            ((vec![-3, 4, 3, 90], 0), vec![0, 2]),
            ((vec![3, 2, 4], 6), vec![1, 2]),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::two_sum(nums, target), expected);
        }
    }
}
