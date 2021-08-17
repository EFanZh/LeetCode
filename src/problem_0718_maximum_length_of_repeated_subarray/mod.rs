pub mod binary_search;

pub trait Solution {
    fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 2, 1] as &[_], &[3, 2, 1, 4, 7] as &[_]), 3),
            ((&[0, 0, 0, 0, 0], &[0, 0, 0, 0, 0]), 5),
            ((&[0, 0, 0, 0, 1], &[1, 0, 0, 0, 0]), 4),
            ((&[1, 2, 3, 2, 1], &[3, 2, 1, 4]), 3),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::find_length(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
