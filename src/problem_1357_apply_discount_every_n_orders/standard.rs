// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct Cashier {
    products: Box<[u16]>,
    sale_factor: u8,
    n_minus_1: u16,
    next_sale: u16,
}

impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let max_product = products.iter().map(|&x| x as u32).max().unwrap();
        let mut internal_products = vec![0; max_product as usize + 1];

        for (product, price) in products.into_iter().zip(prices) {
            internal_products[product as u32 as usize] = price as _;
        }

        let n_minus_1 = n as u16 - 1;

        Self {
            products: internal_products.into_boxed_slice(),
            sale_factor: 100 - (discount as u8),
            n_minus_1,
            next_sale: n_minus_1,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let products = self.products.as_ref();
        let mut base_bill = 0;

        for (i, count) in product.into_iter().zip(amount) {
            base_bill += u32::from(products[i as u32 as usize]) * count as u32;
        }

        if self.next_sale == 0 {
            self.next_sale = self.n_minus_1;

            f64::from(base_bill * u32::from(self.sale_factor)) * 0.01
        } else {
            self.next_sale -= 1;

            f64::from(base_bill)
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Cashier for Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        Self::new(n, discount, products, prices)
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.get_bill(product, amount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Cashier>();
    }
}
