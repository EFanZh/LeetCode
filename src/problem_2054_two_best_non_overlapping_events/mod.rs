pub mod iterative;

pub trait Solution {
    fn max_two_events(events: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 3, 2], [4, 5, 2], [2, 4, 3]] as &[_], 4),
            (&[[1, 3, 2], [4, 5, 2], [1, 5, 5]], 5),
            (&[[1, 5, 3], [1, 5, 1], [6, 6, 5]], 8),
        ];

        for (events, expected) in test_cases {
            assert_eq!(S::max_two_events(events.iter().map(Vec::from).collect()), expected);
        }
    }
}
