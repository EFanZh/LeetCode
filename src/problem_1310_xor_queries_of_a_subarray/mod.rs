pub mod iterative;

pub trait Solution {
    fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 3, 4, 8] as &[_], &[[0, 1], [1, 2], [0, 3], [3, 3]] as &[_]),
                &[2, 7, 14, 8] as &[_],
            ),
            ((&[4, 8, 2, 10], &[[2, 3], [1, 3], [0, 0], [0, 3]]), &[8, 0, 4, 4]),
        ];

        for ((arr, queries), expected) in test_cases {
            assert_eq!(
                S::xor_queries(arr.to_vec(), queries.iter().map(Vec::from).collect()),
                expected
            );
        }
    }
}
