pub mod sliding_window;

pub trait Solution {
    fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 6, 1, 3, 4] as &[_], &[2, 1, 3, 4, 5] as &[_], 25), 3),
            ((&[11, 12, 19], &[10, 8, 7], 19), 0),
        ];

        for ((charge_times, running_costs, budget), expected) in test_cases {
            assert_eq!(
                S::maximum_robots(charge_times.to_vec(), running_costs.to_vec(), budget),
                expected,
            );
        }
    }
}
