pub mod standard;

pub trait Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self;

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Cashier;

    pub fn run<C: Cashier>() {
        let test_cases = [(
            (
                3,
                50,
                &[1, 2, 3, 4, 5, 6, 7] as &[_],
                &[100, 200, 300, 400, 300, 200, 100] as &[_],
            ),
            &[
                ((&[1, 2] as &[_], &[1, 2] as &[_]), 500.0),
                ((&[3, 7], &[10, 10]), 4000.0),
                ((&[1, 2, 3, 4, 5, 6, 7], &[1, 1, 1, 1, 1, 1, 1]), 800.0),
                ((&[4], &[10]), 4000.0),
                ((&[7, 3], &[10, 10]), 4000.0),
                ((&[7, 5, 3, 1, 6, 4, 2], &[10, 10, 10, 9, 9, 9, 7]), 7350.0),
                ((&[2, 3, 5], &[5, 3, 2]), 2500.0),
            ] as &[((&[_], &[_]), _)],
        )];

        for ((n, discount, products, prices), operations) in test_cases {
            let mut cashier = C::new(n, discount, products.to_vec(), prices.to_vec());

            for &((product, amount), expected) in operations {
                approx::assert_ulps_eq!(cashier.get_bill(product.to_vec(), amount.to_vec()), expected);
            }
        }
    }
}
