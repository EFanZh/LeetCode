pub mod iterative;

pub trait Solution {
    fn closest_divisors(num: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (8, [3, 3]),
            (123, [5, 25]),
            (999, [25, 40]),
            (208_656_121, [39, 5_350_157]),
        ];

        for (num, expected) in test_cases {
            assert_eq!(test_utilities::unstable_sorted(S::closest_divisors(num)), expected);
        }
    }
}
