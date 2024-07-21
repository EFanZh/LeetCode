pub mod memoized_dynamic_programming;

pub trait Solution {
    fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 0, &[-1] as &[_], &[0] as &[_]), 0),
            ((6, 2, &[2, 2, -1, 2, 2, 2], &[0, 0, 1, 0, 0, 0]), 1),
        ];

        for ((n, head_id, manager, inform_time), expected) in test_cases {
            assert_eq!(
                S::num_of_minutes(n, head_id, manager.to_vec(), inform_time.to_vec()),
                expected,
            );
        }
    }
}
