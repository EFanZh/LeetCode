pub mod brute_force;
pub mod brute_force_2;
pub mod brute_force_3;

pub trait Solution {
    fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 1], [1, 0], [0, 1]] as &[_], 2.0),
            (&[[0, 1], [2, 1], [1, 1], [1, 0], [2, 0]], 1.0),
            (&[[0, 3], [1, 2], [3, 1], [1, 3], [2, 1]], 0.0),
            (&[[3, 1], [1, 1], [0, 1], [2, 1], [3, 3], [3, 2], [0, 2], [2, 3]], 2.0),
        ];

        for (points, expected) in test_cases {
            approx::assert_ulps_eq!(S::min_area_free_rect(points.iter().map(Vec::from).collect()), expected);
        }
    }
}
