pub mod iterative;

pub trait Solution {
    fn min_operations_max_profit(customers: Vec<i32>, boarding_cost: i32, running_cost: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[8, 3] as &[_], 5, 6), 3),
            ((&[10, 9, 6], 6, 4), 7),
            ((&[3, 4, 0, 1], 1, 92), -1),
            ((&[5, 0, 0, 0, 0, 30], 5, 5), 13),
            ((&[10, 10, 6, 4, 7], 3, 8), 9),
            ((&[4, 4, 0, 0, 0, 6], 3, 5), 2),
        ];

        for ((customers, boarding_cost, running_cost), expected) in test_cases {
            assert_eq!(
                S::min_operations_max_profit(customers.to_vec(), boarding_cost, running_cost),
                expected,
            );
        }
    }
}
