pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut n = n as u32;
        let mut result = 0;

        n >>= n.trailing_zeros();

        while n != 1 {
            n -= 1;

            let trailing_zeros = n.trailing_zeros();

            result = result.max(trailing_zeros);
            n >>= trailing_zeros;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn binary_gap(n: i32) -> i32 {
        Self::binary_gap(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
