pub mod backtracking;
pub mod dynamic_programming;

pub trait Solution {
    fn diff_ways_to_compute(input: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("2-1-1", &[0, 2] as &[_]),
            ("2*3-4*5", &[-34, -14, -10, -10, 10]),
            ("11", &[11]),
            ("0+1", &[1]),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::diff_ways_to_compute(input.to_string())),
                expected
            );
        }
    }
}
