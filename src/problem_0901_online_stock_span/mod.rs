pub mod monotonic_stack;

pub trait StockSpanner {
    fn new() -> Self;
    fn next(&mut self, price: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::StockSpanner;

    pub fn run<S: StockSpanner>() {
        let test_cases = [&[(100, 1), (80, 1), (60, 1), (70, 2), (60, 1), (75, 4), (85, 6)] as &[_]];

        for operations in test_cases {
            let mut stock_spanner = S::new();

            for &(price, expected) in operations {
                assert_eq!(stock_spanner.next(price), expected);
            }
        }
    }
}
