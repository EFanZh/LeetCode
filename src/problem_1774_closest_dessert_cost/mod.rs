pub mod binary_search;

pub trait Solution {
    fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 7] as &[_], &[3, 4] as &[_], 10), 10),
            ((&[2, 3], &[4, 5, 100], 18), 17),
            ((&[3, 10], &[2, 5], 9), 8),
            ((&[10], &[1], 1), 10),
            ((&[3], &[3], 9), 9),
            (
                (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1000),
                120,
            ),
        ];

        for ((base_costs, topping_costs, target), expected) in test_cases {
            assert_eq!(
                S::closest_cost(base_costs.to_vec(), topping_costs.to_vec(), target),
                expected,
            );
        }
    }
}
