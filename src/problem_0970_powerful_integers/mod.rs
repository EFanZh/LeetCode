pub mod brute_force;

pub trait Solution {
    fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, 3, 10), &[2, 3, 4, 5, 7, 9, 10] as &[_]),
            ((3, 5, 15), &[2, 4, 6, 8, 10, 14]),
            ((1, 1, 400_000), &[2]),
            ((1, 2, 100), &[2, 3, 5, 9, 17, 33, 65]),
            ((2, 1, 10), &[2, 3, 5, 9]),
        ];

        for ((x, y, bound), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::powerful_integers(x, y, bound)),
                expected
            );
        }
    }
}
