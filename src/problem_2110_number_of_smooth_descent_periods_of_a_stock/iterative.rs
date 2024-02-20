pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut prev = 0;
        let mut length = 0_u64;
        let mut result = 0;

        for price in prices {
            if price + 1 == prev {
                length += 1;
            } else {
                length = 1;
            }

            result += length;
            prev = price;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_descent_periods(prices: Vec<i32>) -> i64 {
        Self::get_descent_periods(prices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
