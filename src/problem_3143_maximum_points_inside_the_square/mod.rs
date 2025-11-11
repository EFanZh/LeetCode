pub mod quick_select;

pub trait Solution {
    fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[2, 2], [-1, -2], [-4, 4], [-3, 1], [3, -3]] as &[_], "abdca"), 2),
            ((&[[1, 1], [-2, -2], [-2, 2]], "abb"), 1),
            ((&[[1, 1], [-1, -1], [2, -2]], "ccd"), 0),
        ];

        for ((points, s), expected) in test_cases {
            assert_eq!(
                S::max_points_inside_square(points.iter().map(Vec::from).collect(), s.to_string()),
                expected,
            );
        }
    }
}
