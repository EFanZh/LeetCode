pub mod iterative;

pub trait Solution {
    fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], &[2, 4] as &[_]), 2),
            ((&[1, 2, 3, 6], &[2, 3, 4, 5]), 2),
            ((&[2, 4], &[1, 2]), 2),
            ((&[1, 2, 3, 4], &[5, 6, 7, 8]), -1),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::get_common(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
