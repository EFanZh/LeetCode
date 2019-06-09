pub mod binary_search;
pub mod fast_binary_search;
pub mod simplified_binary_search;
pub mod simplified_three_way_binary_search;
pub mod three_way_binary_search;
pub mod wrapping_fast_binary_search;

pub trait Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = vec![
            ((vec![1, 3], vec![2]), 2.0),
            ((vec![1, 2], vec![3, 4]), 2.5),
            ((vec![1, 2], vec![-1, 3]), 1.5),
            ((vec![100_000], vec![100_001]), 100_000.5),
        ];

        for ((nums1, nums2), expected_result) in test_cases {
            assert!(S::find_median_sorted_arrays(nums1, nums2) - expected_result < std::f64::EPSILON);
        }
    }
}
