pub mod greedy;

pub trait Solution {
    fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 5], [4, 3]] as &[_], 5.0),
            (&[[5, 2], [5, 4], [10, 3], [20, 1]], 3.25),
        ];

        for (customers, expected) in test_cases {
            approx::assert_ulps_eq!(
                S::average_waiting_time(customers.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
