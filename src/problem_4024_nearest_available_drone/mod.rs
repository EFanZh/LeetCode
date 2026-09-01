pub mod greedy;

pub trait Solution {
    fn nearest_drone(drones: Vec<Vec<i32>>, target: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[0, 0, 8], [2, 2, 9]] as &[_], [3, 4]), 1),
            ((&[[2, 1, 5], [4, 4, 5], [6, 6, 8]], [5, 5]), 1),
            ((&[[4, 4, 5]], [8, 6]), -1),
        ];

        for ((drones, target), expected) in test_cases {
            assert_eq!(
                S::nearest_drone(drones.iter().map(Vec::from).collect(), target.to_vec()),
                expected,
            );
        }
    }
}
