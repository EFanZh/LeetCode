pub mod greedy;

pub trait Solution {
    fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 6, 4] as &[_], &[9, 7, 5] as &[_]), 3),
            ((&[10], &[5]), -5),
            ((&[1, 1, 1, 1], &[1, 1, 1, 1]), 0),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::added_integer(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
