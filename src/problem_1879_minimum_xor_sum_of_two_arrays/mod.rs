pub mod dynamic_programming;

pub trait Solution {
    fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2] as &[_], &[2, 3] as &[_]), 2), ((&[1, 0, 3], &[5, 3, 4]), 8)];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::minimum_xor_sum(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
