pub mod dynamic_programming;

pub trait Solution {
    fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 2, 4], [3, 4, 3], [2, 3, 1]] as &[_], 2), 7),
            ((&[[1, 2, 4], [3, 4, 3], [2, 3, 10]], 2), 10),
            ((&[[1, 1, 1], [2, 2, 2], [3, 3, 3], [4, 4, 4]], 3), 9),
            ((&[[30, 40, 34], [6, 11, 6], [60, 81, 36]], 1), 36),
        ];

        for ((events, k), expected) in test_cases {
            assert_eq!(S::max_value(events.iter().map(Vec::from).collect(), k), expected);
        }
    }
}
