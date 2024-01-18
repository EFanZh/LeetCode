pub mod sort_and_binary_search;

pub trait Solution {
    fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[[1, 2], [3, 2], [2, 4], [5, 6], [3, 5]] as &[_],
                    &[1, 2, 3, 4, 5, 6] as &[_],
                ),
                &[2, 4, 5, 5, 6, 6] as &[_],
            ),
            ((&[[1, 2], [1, 2], [1, 3], [1, 4]], &[1]), &[4]),
            ((&[[10, 1000]], &[5]), &[0]),
        ];

        for ((items, queries), expected) in test_cases {
            assert_eq!(
                S::maximum_beauty(items.iter().map(Vec::from).collect(), queries.to_vec()),
                expected,
            );
        }
    }
}
