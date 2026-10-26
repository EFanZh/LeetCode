pub mod disjoint_sets;

pub trait Solution {
    fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (2, &[1, 3] as &[_], 1, &[[0, 0], [0, 1]] as &[_]),
                &[true, false] as &[_],
            ),
            (
                (4, &[2, 5, 6, 8], 2, &[[0, 1], [0, 2], [1, 3], [2, 3]] as &[_]),
                &[false, false, true, true],
            ),
        ];

        for ((n, nums, max_diff, queries), expected) in test_cases {
            assert_eq!(
                S::path_existence_queries(n, nums.to_vec(), max_diff, queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
