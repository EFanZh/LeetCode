pub mod greedy;

pub trait Solution {
    fn min_costs(cost: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[5, 3, 4, 1, 3, 2] as &[_], &[5, 3, 3, 1, 1, 1] as &[_]),
            (&[1, 2, 4, 6, 7], &[1, 1, 1, 1, 1]),
        ];

        for (cost, expected) in test_cases {
            assert_eq!(S::min_costs(cost.to_vec()), expected);
        }
    }
}
