pub mod greedy;

pub trait Solution {
    fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[2, 1], [1, 0], [1, 4], [1, 8], [3, 5], [4, 6]] as &[_], 1), 2),
            ((&[[0, 0], [1, 1], [2, 2], [3, 3], [4, 4], [5, 5], [6, 6]], 2), 3),
            ((&[[2, 3], [1, 2]], 0), 2),
        ];

        for ((points, w), expected) in test_cases {
            assert_eq!(
                S::min_rectangles_to_cover_points(points.iter().map(Vec::from).collect(), w),
                expected,
            );
        }
    }
}
