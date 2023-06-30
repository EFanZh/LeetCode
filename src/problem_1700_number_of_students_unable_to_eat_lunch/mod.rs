pub mod simulation;

pub trait Solution {
    fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1, 0, 0] as &[_], &[0, 1, 0, 1] as &[_]), 0),
            ((&[1, 1, 1, 0, 0, 1], &[1, 0, 0, 0, 1, 1]), 3),
        ];

        for ((students, sandwiches), expected) in test_cases {
            assert_eq!(S::count_students(students.to_vec(), sandwiches.to_vec()), expected);
        }
    }
}
