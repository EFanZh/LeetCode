pub mod exponentiation_by_squaring;

pub trait Solution {
    fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[2, 3, 3, 10], [3, 3, 3, 1], [6, 1, 1, 4]] as &[_], 2),
                &[0, 2] as &[_],
            ),
            ((&[[39, 3, 1000, 1000]], 17), &[]),
        ];

        for ((variables, target), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::get_good_indices(variables.iter().map(Vec::from).collect(), target)),
                expected,
            );
        }
    }
}
