pub mod greedy;

pub trait Solution {
    fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 1, 3] as &[_], &[5, 7] as &[_]), 15),
            ((&[3, 5, 2, 6], &[3, 1, 7]), 3),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::min_number(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
