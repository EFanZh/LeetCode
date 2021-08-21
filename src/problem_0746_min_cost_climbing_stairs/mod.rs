pub mod dynamic_programming;

pub trait Solution {
    fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[10, 15, 20] as &[_], 15), (&[1, 100, 1, 1, 1, 100, 1, 1, 100, 1], 6)];

        for (cost, expected) in test_cases {
            assert_eq!(S::min_cost_climbing_stairs(cost.to_vec()), expected);
        }
    }
}
