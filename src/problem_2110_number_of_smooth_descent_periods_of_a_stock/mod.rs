pub mod iterative;

pub trait Solution {
    fn get_descent_periods(prices: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 2, 1, 4] as &[_], 7), (&[8, 6, 7, 7], 4), (&[1], 1)];

        for (prices, expected) in test_cases {
            assert_eq!(S::get_descent_periods(prices.to_vec()), expected);
        }
    }
}
