pub mod iterative;

pub trait Solution {
    fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((10, &[[0, 3], [2, 5], [0, 9], [1, 15]] as &[_]), 1),
            ((26, &[[1, 1], [3, 7], [2, 12], [7, 17]]), 3),
            ((2, &[[0, 10], [1, 20]]), 0),
            (
                (70, &[[36, 3], [1, 5], [12, 8], [25, 9], [53, 11], [29, 12], [52, 14]]),
                12,
            ),
        ];

        for ((n, logs), expected) in test_cases {
            assert_eq!(S::hardest_worker(n, logs.iter().map(Vec::from).collect()), expected);
        }
    }
}
