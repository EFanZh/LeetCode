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

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3] as &[_], &[2] as &[_]), 2.0),
            ((&[1, 2], &[3, 4]), 2.5),
            ((&[1, 2], &[-1, 3]), 1.5),
            ((&[100_000], &[100_001]), 100_000.5),
            ((&[3, 4], &[1, 2]), 2.5),
            ((&[1, 2], &[1, 2, 3]), 2.0),
            ((&[1], &[2, 3, 4]), 2.5),
        ];

        for ((nums1, nums2), expected_result) in test_cases.iter().copied() {
            approx::assert_relative_eq!(
                S::find_median_sorted_arrays(nums1.to_vec(), nums2.to_vec()),
                expected_result,
            );
        }
    }
}
