pub mod stack;

pub trait Solution {
    fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 2] as &[_], &[1, 2, 3, 3] as &[_]),
            (&[2, 2, 1], &[1, 2, 1]),
            (&[3, 1, 5, 6, 4, 2], &[1, 1, 2, 3, 2, 2]),
            (&[5, 1, 5, 5, 1, 3, 4, 5, 1, 4], &[1, 1, 2, 3, 2, 3, 4, 5, 3, 5]),
        ];

        for (obstacles, expected) in test_cases {
            assert_eq!(
                S::longest_obstacle_course_at_each_position(obstacles.to_vec()),
                expected,
            );
        }
    }
}
