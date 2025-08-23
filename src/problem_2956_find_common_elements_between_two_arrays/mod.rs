pub mod brute_force;

pub trait Solution {
    fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 3, 2] as &[_], &[1, 2] as &[_]), [2, 1]),
            ((&[4, 3, 2, 3, 1], &[2, 2, 5, 2, 3, 6]), [3, 4]),
            ((&[3, 4, 2, 3], &[1, 5]), [0, 0]),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::find_intersection_values(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
