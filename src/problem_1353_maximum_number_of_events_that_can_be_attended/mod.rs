pub mod greedy_binary_heap;

pub trait Solution {
    fn max_events(events: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 3], [3, 4]] as &[_], 3),
            (&[[1, 2], [2, 3], [3, 4], [1, 2]], 4),
            (&[[1, 4], [4, 4], [2, 2], [3, 4], [1, 1]], 4),
            (&[[1, 2], [1, 2], [3, 3], [1, 5], [1, 5]], 5),
            (
                &[
                    [27, 27],
                    [8, 10],
                    [9, 11],
                    [20, 21],
                    [25, 29],
                    [17, 20],
                    [12, 12],
                    [12, 12],
                    [10, 14],
                    [7, 7],
                    [6, 10],
                    [7, 7],
                    [4, 8],
                    [30, 31],
                    [23, 25],
                    [4, 6],
                    [17, 17],
                    [13, 14],
                    [6, 9],
                    [13, 14],
                ],
                18,
            ),
        ];

        for (events, expected) in test_cases {
            assert_eq!(S::max_events(events.iter().map(Vec::from).collect()), expected);
        }
    }
}
