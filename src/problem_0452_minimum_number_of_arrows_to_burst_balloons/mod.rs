pub mod greedy;

pub trait Solution {
    fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[10, 16], [2, 8], [1, 6], [7, 12]] as &[_], 2),
            (&[[1, 2], [3, 4], [5, 6], [7, 8]], 4),
            (&[[1, 2], [2, 3], [3, 4], [4, 5]], 2),
            (&[[1, 2]], 1),
            (&[[2, 3], [2, 3]], 1),
        ];

        for (points, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::find_min_arrow_shots(points.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
