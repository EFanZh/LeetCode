pub mod dynamic_programming;

pub trait Solution {
    fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], 3), 2),
            ((&[3, 1, 3, 1, 1], 8), 2),
            ((&[1, 2, 3, 4, 5], 15), 1),
        ];

        for ((tasks, session_time), expected) in test_cases {
            assert_eq!(S::min_sessions(tasks.to_vec(), session_time), expected);
        }
    }
}
