pub mod binary_heap_and_binary_search;

pub trait Solution {
    fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[1, 4], [2, 4], [3, 6], [4, 4]] as &[_], &[2, 3, 4, 5] as &[_]),
                &[3, 3, 1, 4] as &[_],
            ),
            ((&[[2, 3], [2, 5], [1, 8], [20, 25]], &[2, 19, 5, 22]), &[2, -1, 4, 6]),
        ];

        for ((intervals, queries), expected) in test_cases {
            assert_eq!(
                S::min_interval(intervals.iter().map(Vec::from).collect(), queries.to_vec()),
                expected,
            );
        }
    }
}
