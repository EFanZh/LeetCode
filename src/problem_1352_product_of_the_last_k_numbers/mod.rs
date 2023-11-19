pub mod cumulative_product;

pub trait ProductOfNumbers {
    fn new() -> Self;
    fn add(&mut self, num: i32);
    fn get_product(&self, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::ProductOfNumbers;

    enum Operation {
        Add(i32),
        GetProduct(i32, i32),
    }

    pub fn run<P: ProductOfNumbers>() {
        let test_cases = [
            &[
                Operation::Add(3),
                Operation::Add(0),
                Operation::Add(2),
                Operation::Add(5),
                Operation::Add(4),
                Operation::GetProduct(2, 20),
                Operation::GetProduct(3, 40),
                Operation::GetProduct(4, 0),
                Operation::Add(8),
                Operation::GetProduct(2, 32),
            ] as &[_],
            &[Operation::Add(0), Operation::GetProduct(1, 0)],
        ];

        for operations in test_cases {
            let mut product_of_numbers = P::new();

            for operation in operations {
                match *operation {
                    Operation::Add(num) => product_of_numbers.add(num),
                    Operation::GetProduct(k, expected) => assert_eq!(product_of_numbers.get_product(k), expected),
                }
            }
        }
    }
}
