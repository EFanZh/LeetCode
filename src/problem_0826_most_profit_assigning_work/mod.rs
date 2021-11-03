pub mod iterative;

pub trait Solution {
    fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[2, 4, 6, 8, 10] as &[_],
                    &[10, 20, 30, 40, 50] as &[_],
                    &[4, 5, 6, 7] as &[_],
                ),
                100,
            ),
            ((&[85, 47, 57], &[24, 66, 99], &[40, 25, 25]), 0),
            ((&[13, 37, 58], &[4, 90, 96], &[34, 73, 45]), 190),
        ];

        for ((difficulty, profit, worker), expected) in test_cases {
            assert_eq!(
                S::max_profit_assignment(difficulty.to_vec(), profit.to_vec(), worker.to_vec()),
                expected
            );
        }
    }
}
