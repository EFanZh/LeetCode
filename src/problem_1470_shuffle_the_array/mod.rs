pub mod quick_select;

pub trait Solution {
    fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 2), &[1, 5] as &[_]),
            ((&[1, 1, 3, 5, 5], 2), &[5, 5]),
            ((&[6, 7, 11, 7, 6, 8], 5), &[6, 6, 7, 8, 11]),
            ((&[513], 1), &[513]),
        ];

        for ((arr, k), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::get_strongest(arr.to_vec(), k)),
                expected
            );
        }
    }
}
