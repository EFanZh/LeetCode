pub mod brute_force;

pub trait Solution {
    fn max_points(points: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 1], [2, 2], [3, 3]] as &[_], 3),
            (&[[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]], 4),
        ];

        for (points, expected) in test_cases {
            assert_eq!(S::max_points(points.iter().copied().map(Vec::from).collect()), expected);
        }
    }
}
