pub mod check_gradient;

pub trait Solution {
    fn is_boomerang(points: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ([[1, 1], [2, 3], [3, 2]], true),
            ([[1, 1], [2, 2], [3, 3]], false),
            ([[0, 0], [0, 2], [2, 1]], true),
        ];

        for (points, expected) in test_cases {
            assert_eq!(S::is_boomerang(points.into_iter().map(Vec::from).collect()), expected);
        }
    }
}
