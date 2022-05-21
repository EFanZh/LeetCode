pub mod even_odd;

pub trait Solution {
    fn rearrange_barcodes(arr: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [&[1, 1, 1, 2, 2, 2] as &[_], &[1, 1, 1, 1, 2, 2, 3, 3]];

        for arr in test_cases {
            let result = S::rearrange_barcodes(arr.to_vec());

            assert_eq!(
                test_utilities::unstable_sorted(result.iter().copied()),
                test_utilities::unstable_sorted(arr.iter().copied())
            );

            for (lhs, rhs) in result.iter().zip(&result[1..]) {
                assert_ne!(lhs, rhs);
            }
        }
    }
}
