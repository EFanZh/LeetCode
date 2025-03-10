pub mod dfs;

pub trait Solution {
    fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[0, 1], [0, 2], [0, 3]] as &[_], 5), 3),
            ((&[[3, 1], [3, 2], [1, 0], [0, 4], [0, 5], [4, 6]], 2), 7),
        ];

        for ((roads, seats), expected) in test_cases {
            assert_eq!(
                S::minimum_fuel_cost(roads.iter().map(Vec::from).collect(), seats),
                expected,
            );
        }
    }
}
