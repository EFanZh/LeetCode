pub mod iterative;

pub trait Solution {
    fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 3] as &[_], &[3, 2, 7] as &[_], 4), 1), ((&[4], &[4], 4), 1)];

        for ((start_time, end_time, query_time), expected) in test_cases {
            assert_eq!(
                S::busy_student(start_time.to_vec(), end_time.to_vec(), query_time),
                expected,
            );
        }
    }
}
