pub mod hash_map_and_stack;

pub trait Solution {
    fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 1, 2] as &[_], &[1, 3, 4, 2] as &[_]), &[-1, 3, -1] as &[_]),
            ((&[2, 4], &[1, 2, 3, 4]), &[3, -1]),
        ];

        for ((nums1, nums2), expected) in test_cases.iter().copied() {
            assert_eq!(S::next_greater_element(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
