pub mod brute_force;

pub trait Solution {
    fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 2, 5], [2, 1, 7], [3, 1, 9]] as &[_], 2), [2, 1]),
            ((&[[23, 11, 21]], 9), [23, 11]),
            ((&[[1, 2, 13], [2, 1, 7], [0, 1, 9]], 2), [1, 2]),
        ];

        for ((towers, radius), expected) in test_cases {
            assert_eq!(
                S::best_coordinate(towers.iter().copied().map(Vec::from).collect(), radius),
                expected,
            );
        }
    }
}
