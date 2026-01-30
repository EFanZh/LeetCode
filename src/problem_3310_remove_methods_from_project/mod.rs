pub mod dfs;

pub trait Solution {
    fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, 1, &[[1, 2], [0, 1], [3, 2]] as &[_]), &[0, 1, 2, 3] as &[_]),
            ((5, 0, &[[1, 2], [0, 2], [0, 1], [3, 4]]), &[3, 4]),
            ((3, 2, &[[1, 2], [0, 1], [2, 0]]), &[]),
        ];

        for ((n, k, invocations), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::remaining_methods(
                    n,
                    k,
                    invocations.iter().map(Vec::from).collect(),
                )),
                expected,
            );
        }
    }
}
