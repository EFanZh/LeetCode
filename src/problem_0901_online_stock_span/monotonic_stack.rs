// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct StockSpanner {
    stack: Vec<(u32, i32)>,
}

impl StockSpanner {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn next(&mut self, price: i32) -> i32 {
        let stack = &mut self.stack;
        let mut result = 1;

        while let Some(&(span, day_price)) = stack.last() {
            if price < day_price {
                break;
            }

            result += span;
            stack.pop();
        }

        stack.push((result, price));

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::StockSpanner for StockSpanner {
    fn new() -> Self {
        Self::new()
    }

    fn next(&mut self, price: i32) -> i32 {
        self.next(price)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::StockSpanner>();
    }
}
