pub mod iterative;

pub trait Solution {
    fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 2), &[3, 4] as &[_]),
            ((&[10, 1, 10, 1, 10], 3), &[1, 3]),
            ((&[10, 1, 10, 1, 10], 10), &[]),
        ];

        for ((height, threshold), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::stable_mountains(height.to_vec(), threshold)),
                expected,
            );
        }
    }
}
