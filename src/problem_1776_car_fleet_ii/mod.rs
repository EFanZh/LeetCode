pub mod monotonic_stack;

pub trait Solution {
    fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 2], [2, 1], [4, 3], [7, 2]] as &[_],
                &[1.0, -1.0, 3.0, -1.0] as &[_],
            ),
            (&[[3, 4], [5, 4], [6, 3], [9, 1]], &[2.0, 1.0, 1.5, -1.0]),
        ];

        for (cars, expected) in test_cases {
            assert_eq!(S::get_collision_times(cars.iter().map(Vec::from).collect()), expected);
        }
    }
}
