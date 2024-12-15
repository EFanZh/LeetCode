pub mod iterative;

pub trait Solution {
    fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[1, 1], [4, 5], [3, 8]] as &[_], &[[3, 1], [1, 5]] as &[_]),
                &[[1, 6], [3, 9], [4, 5]] as &[_],
            ),
            (
                (&[[1, 1], [3, 2], [2, 3]], &[[2, 1], [3, 2], [1, 3]]),
                &[[1, 4], [2, 4], [3, 4]],
            ),
            (
                (&[[1, 3], [2, 2]], &[[7, 1], [2, 2], [1, 4]]),
                &[[1, 7], [2, 4], [7, 1]],
            ),
        ];

        for ((items1, items2), expected) in test_cases {
            assert_eq!(
                S::merge_similar_items(
                    items1.iter().map(Vec::from).collect(),
                    items2.iter().map(Vec::from).collect(),
                ),
                expected,
            );
        }
    }
}
