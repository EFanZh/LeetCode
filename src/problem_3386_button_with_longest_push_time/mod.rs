pub mod iterative;

pub trait Solution {
    fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 5], [3, 9], [1, 15]] as &[_], 1),
            (&[[10, 5], [1, 7]], 10),
        ];

        for (events, expected) in test_cases {
            assert_eq!(
                S::button_with_longest_time(events.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
