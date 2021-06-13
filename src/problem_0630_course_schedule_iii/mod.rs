pub mod binary_heap;

pub trait Solution {
    fn schedule_course(courses: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[100, 200], [200, 1300], [1000, 1250], [2000, 3200]] as &[_], 3),
            (&[[1, 2]], 1),
            (&[[3, 2], [4, 3]], 0),
            (&[[1, 2], [2, 3]], 2),
            (&[[5, 5], [4, 6], [2, 6]], 2),
        ];

        for (courses, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::schedule_course(courses.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
