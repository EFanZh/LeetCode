pub mod greedy;

pub trait Solution {
    fn max_weight(pizzas: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5, 6, 7, 8] as &[_], 14),
            (&[2, 1, 1, 1, 1, 1, 1, 1], 3),
            (&[8, 6, 10, 5, 4, 4, 8, 4, 9, 9, 2, 1, 4, 3, 2, 1, 7, 3, 8, 6], 43),
        ];

        for (pizzas, expected) in test_cases {
            assert_eq!(S::max_weight(pizzas.to_vec()), expected);
        }
    }
}
