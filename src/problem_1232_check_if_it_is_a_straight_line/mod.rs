pub mod check_gradient;

pub trait Solution {
    fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 7]] as &[_], true),
            (&[[1, 1], [2, 2], [3, 4], [4, 5], [5, 6], [7, 7]], false),
        ];

        for (coordinates, expected) in test_cases {
            assert_eq!(
                S::check_straight_line(coordinates.iter().map(Vec::from).collect()),
                expected
            );
        }
    }
}
