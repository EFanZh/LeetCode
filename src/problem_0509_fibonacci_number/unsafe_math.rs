pub struct Solution {}

// https://en.wikipedia.org/wiki/Fibonacci_number#Closed-form_expression

impl Solution {
    const PHI: f64 = 1.618_033_988_749_895;
    const PSI: f64 = -0.618_033_988_749_894_9;
    const SQRT_5: f64 = 2.236_067_977_499_79;

    pub fn fib(n: i32) -> i32 {
        ((Self::PHI.powi(n) - Self::PSI.powi(n)) / Self::SQRT_5).round() as _
    }
}

impl super::Solution for Solution {
    fn fib(n: i32) -> i32 {
        Self::fib(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
