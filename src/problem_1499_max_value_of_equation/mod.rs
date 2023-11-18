pub mod monotonic_stack;

pub trait Solution {
    fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 3], [2, 0], [5, 10], [6, -10]] as &[_], 1), 4),
            ((&[[0, 0], [3, 0], [9, 2]], 3), 3),
            ((&[[-19, 9], [-15, -19], [-5, -8]], 10), -6),
        ];

        for ((points, k), expected) in test_cases {
            assert_eq!(
                S::find_max_value_of_equation(points.iter().map(Vec::from).collect(), k),
                expected,
            );
        }
    }
}
