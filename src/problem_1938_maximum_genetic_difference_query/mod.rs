pub mod trie;

pub trait Solution {
    fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[-1, 0, 1, 1] as &[_], &[[0, 2], [3, 2], [2, 5]] as &[_]),
                &[2, 3, 7] as &[_],
            ),
            ((&[3, 7, -1, 2, 0, 7, 0, 2], &[[4, 6], [1, 15], [0, 5]]), &[6, 14, 7]),
            (
                (&[-1, 0, 0, 0, 3], &[[4, 6], [0, 0], [0, 3], [1, 8], [4, 0]]),
                &[6, 0, 3, 9, 4],
            ),
        ];

        for ((parents, queries), expected) in test_cases {
            assert_eq!(
                S::max_genetic_difference(parents.to_vec(), queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
