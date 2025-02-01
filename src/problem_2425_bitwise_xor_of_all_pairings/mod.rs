pub mod mathematical;

pub trait Solution {
    fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 1, 3] as &[_], &[10, 2, 5, 0] as &[_]), 13),
            ((&[1, 2], &[3, 4]), 0),
            ((&[8, 6, 29, 2, 26, 16, 15, 29], &[24, 12, 12]), 9),
            ((&[25, 29, 3, 10, 0, 15, 2], &[12]), 12),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::xor_all_nums(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
