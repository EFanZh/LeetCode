pub mod iterative;

pub trait Solution {
    fn min_price(prices: Vec<i32>, discounts: Vec<i32>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[10, 30, 21] as &[_], &[50, 60] as &[_]), 32.5),
            ((&[100, 70], &[10, 40, 50]), 92.0),
            ((&[7, 3, 9], &[100, 100]), 3.0),
        ];

        for ((prices, discounts), expected) in test_cases {
            approx::assert_ulps_eq!(S::min_price(prices.to_vec(), discounts.to_vec()), expected);
        }
    }
}
