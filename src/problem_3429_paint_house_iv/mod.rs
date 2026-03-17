pub mod dynamic_programming;

pub trait Solution {
    fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[3, 5, 7], [6, 2, 9], [4, 8, 1], [7, 3, 5]] as &[_]), 9),
            (
                (6, &[[2, 4, 6], [5, 3, 8], [7, 1, 9], [4, 6, 2], [3, 5, 7], [8, 2, 4]]),
                18,
            ),
        ];

        for ((n, cost), expected) in test_cases {
            assert_eq!(S::min_cost(n, cost.iter().map(Vec::from).collect()), expected);
        }
    }
}
