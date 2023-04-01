pub mod dynamic_programming;

pub trait Solution {
    fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 1, -2, 5] as &[_], &[3, 0, -6] as &[_]), 18),
            ((&[3, -2], &[2, -6, 7]), 21),
            ((&[-1, -1], &[1, 1]), -1),
            ((&[-3, -8, 3, -10, 1, 3, 9], &[9, 2, 3, 7, -9, 1, -8, 5, -1, -1]), 200),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::max_dot_product(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
