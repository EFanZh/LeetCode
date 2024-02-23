pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn minimum_cost(cost: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3] as &[_], 5),
            (&[6, 5, 7, 9, 2, 2], 23),
            (&[5, 5], 10),
            (&[3, 3, 3, 1], 7),
            (&[1], 1),
        ];

        for (cost, expected) in test_cases {
            assert_eq!(S::minimum_cost(cost.to_vec()), expected);
        }
    }
}
