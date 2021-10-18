pub mod dynamic_programming;

pub trait Solution {
    fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 5, 4] as &[_], &[1, 2, 3, 7] as &[_]), 1),
            ((&[0, 3, 5, 8, 9], &[2, 1, 4, 6, 9]), 1),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::min_swap(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
