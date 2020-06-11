pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            (&[1, 2, 3, 0, 0, 0] as &[_], 3, &[2, 5, 6] as &[_], 3),
            &[1, 2, 2, 3, 5, 6] as &[_],
        )];

        for ((nums1, m, nums2, n), expected) in test_cases.iter().copied() {
            let mut nums1 = nums1.to_vec();

            S::merge(&mut nums1, m, &mut nums2.to_vec(), n);

            assert_eq!(&nums1[..(m + n) as _], expected);
        }
    }
}
