pub mod dfs;
pub mod interned_dfs;
pub mod interned_dfs_with_stack;

pub trait Solution {
    fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            &[[5, 1], [4, 5], [11, 9], [9, 4]] as &[_],
            &[[1, 3], [3, 2], [2, 1]],
            &[[1, 2], [1, 3], [2, 1]],
            &[
                [5, 13],
                [10, 6],
                [11, 3],
                [15, 19],
                [16, 19],
                [1, 10],
                [19, 11],
                [4, 16],
                [19, 9],
                [5, 11],
                [5, 6],
                [13, 5],
                [13, 9],
                [9, 15],
                [11, 16],
                [6, 9],
                [9, 13],
                [3, 1],
                [16, 5],
                [6, 5],
            ],
        ];

        for pairs in test_cases {
            let result = S::valid_arrangement(pairs.iter().map(Vec::from).collect());

            for window in result.windows(2) {
                assert_eq!(window[0][1], window[1][0]);
            }

            assert_eq!(
                test_utilities::unstable_sorted(result),
                test_utilities::unstable_sorted(pairs),
            );
        }
    }
}
