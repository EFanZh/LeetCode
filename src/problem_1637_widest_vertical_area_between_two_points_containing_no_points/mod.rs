pub mod greedy;

pub trait Solution {
    fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[8, 7], [9, 9], [7, 4], [9, 7]] as &[_], 1),
            (&[[3, 1], [9, 0], [1, 0], [1, 4], [5, 3], [8, 8]], 3),
        ];

        for (points, expected) in test_cases {
            assert_eq!(
                S::max_width_of_vertical_area(points.iter().map(Vec::from).collect()),
                expected
            );
        }
    }
}
