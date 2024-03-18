pub mod hash_set;

pub trait Solution {
    fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 2, 3] as &[_], &[2, 4, 6] as &[_]),
                [&[1, 3] as &[_], &[4, 6] as &[_]],
            ),
            ((&[1, 2, 3, 3], &[1, 1, 2, 2]), [&[3] as &[_], &[] as &[_]]),
        ];

        for ((nums1, nums2), expected) in test_cases {
            let mut result = S::find_difference(nums1.to_vec(), nums2.to_vec());

            for value in &mut result {
                value.sort_unstable();
            }

            assert_eq!(result, expected.map(Vec::from),);
        }
    }
}
