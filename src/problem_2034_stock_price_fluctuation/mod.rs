pub mod binary_heap_and_hash_map;

pub trait StockPrice {
    fn new() -> Self;
    fn update(&mut self, timestamp: i32, price: i32);
    fn current(&self) -> i32;
    fn maximum(&mut self) -> i32;
    fn minimum(&mut self) -> i32;
}

#[cfg(test)]
mod tests {
    use super::StockPrice;

    enum Operation {
        Update(i32, i32),
        Current(i32),
        Maximum(i32),
        Minimum(i32),
    }

    pub fn run<S: StockPrice>() {
        let test_cases = [
            &[
                Operation::Update(1, 10),
                Operation::Update(2, 5),
                Operation::Current(5),
                Operation::Maximum(10),
                Operation::Update(1, 3),
                Operation::Maximum(5),
                Operation::Update(4, 2),
                Operation::Minimum(2),
            ] as &[_],
            &[
                Operation::Update(38, 2308),
                Operation::Maximum(2308),
                Operation::Current(2308),
                Operation::Minimum(2308),
                Operation::Maximum(2308),
                Operation::Maximum(2308),
                Operation::Maximum(2308),
                Operation::Minimum(2308),
                Operation::Minimum(2308),
                Operation::Maximum(2308),
                Operation::Update(47, 7876),
                Operation::Maximum(7876),
                Operation::Minimum(2308),
                Operation::Update(58, 1866),
                Operation::Maximum(7876),
                Operation::Minimum(1866),
                Operation::Current(1866),
                Operation::Maximum(7876),
                Operation::Update(43, 121),
                Operation::Minimum(121),
                Operation::Maximum(7876),
                Operation::Update(40, 5339),
                Operation::Maximum(7876),
                Operation::Maximum(7876),
                Operation::Current(1866),
                Operation::Update(32, 5339),
                Operation::Current(1866),
                Operation::Minimum(121),
                Operation::Update(43, 6414),
                Operation::Update(49, 9369),
                Operation::Minimum(1866),
                Operation::Minimum(1866),
                Operation::Update(36, 3192),
                Operation::Current(1866),
                Operation::Update(48, 1006),
                Operation::Maximum(9369),
                Operation::Update(53, 8013),
                Operation::Minimum(1006),
            ],
        ];

        for operations in test_cases {
            let mut stock_price = S::new();

            for operation in operations {
                match *operation {
                    Operation::Update(timestamp, price) => stock_price.update(timestamp, price),
                    Operation::Current(expected) => assert_eq!(stock_price.current(), expected),
                    Operation::Maximum(expected) => assert_eq!(stock_price.maximum(), expected),
                    Operation::Minimum(expected) => assert_eq!(stock_price.minimum(), expected),
                }
            }
        }
    }
}
