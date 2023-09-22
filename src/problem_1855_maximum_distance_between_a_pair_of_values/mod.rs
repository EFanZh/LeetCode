pub mod two_pointers;
pub mod two_pointers_2;

pub trait Solution {
    fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[55, 30, 5, 4, 2] as &[_], &[100, 20, 10, 10, 5] as &[_]), 2),
            ((&[2, 2, 2], &[10, 10, 1]), 1),
            ((&[30, 29, 19, 5], &[25, 25, 25, 25, 25]), 2),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::max_distance(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
