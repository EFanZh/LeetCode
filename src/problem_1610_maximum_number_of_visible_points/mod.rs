pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[2, 1], [2, 2], [3, 3]] as &[_], 90, [1, 1]), 3),
            ((&[[2, 1], [2, 2], [3, 4], [1, 1]], 90, [1, 1]), 4),
            ((&[[1, 0], [2, 1]], 13, [1, 1]), 1),
            ((&[[1, 1], [2, 2], [3, 3], [4, 4], [1, 2], [2, 1]], 0, [1, 1]), 4),
            ((&[[1, 1], [1, 1], [1, 1]], 1, [1, 1]), 3),
            ((&[[0, 0], [0, 2]], 90, [1, 1]), 2),
        ];

        for ((points, angle, location), expected) in test_cases {
            assert_eq!(
                S::visible_points(
                    points.iter().copied().map(Vec::from).collect(),
                    angle,
                    location.to_vec()
                ),
                expected,
            );
        }
    }
}
