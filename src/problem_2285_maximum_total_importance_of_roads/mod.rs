pub mod greedy;

pub trait Solution {
    fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, &[[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]] as &[_]), 43),
            ((5, &[[0, 3], [2, 4], [1, 3]]), 20),
        ];

        for ((n, roads), expected) in test_cases {
            assert_eq!(
                S::maximum_importance(n, roads.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
