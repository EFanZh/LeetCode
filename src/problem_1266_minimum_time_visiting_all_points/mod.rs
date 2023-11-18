pub mod greedy;

pub trait Solution {
    fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[[1, 1], [3, 4], [-1, 0]] as &[_], 7), (&[[3, 2], [-2, 2]], 5)];

        for (points, expected) in test_cases {
            assert_eq!(
                S::min_time_to_visit_all_points(points.iter().map(Vec::from).collect()),
                expected
            );
        }
    }
}
