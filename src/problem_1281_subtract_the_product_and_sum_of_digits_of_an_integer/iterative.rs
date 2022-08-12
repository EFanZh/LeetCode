pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut n = n as u32;
        let mut product = 1;
        let mut sum = 0;

        loop {
            let digit = (n % 10) as i32;

            n /= 10;
            product *= digit;
            sum += digit;

            if n == 0 {
                break;
            }
        }

        product - sum
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn subtract_product_and_sum(n: i32) -> i32 {
        Self::subtract_product_and_sum(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
