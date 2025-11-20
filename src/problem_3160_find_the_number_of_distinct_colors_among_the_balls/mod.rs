pub mod hash_map;

pub trait Solution {
    fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[1, 4], [2, 5], [1, 3], [3, 4]] as &[_]), &[1, 2, 2, 3] as &[_]),
            ((4, &[[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]]), &[1, 2, 2, 3, 4]),
        ];

        for ((limit, queries), expected) in test_cases {
            assert_eq!(
                S::query_results(limit, queries.iter().map(Vec::from).collect()),
                expected
            );
        }
    }
}
