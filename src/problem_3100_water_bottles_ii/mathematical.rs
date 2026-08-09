pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let b = num_bottles.cast_unsigned();
        let e = num_exchange.cast_unsigned();
        let t = e.wrapping_mul(2).wrapping_sub(3); // 2e - 3.

        (b + ((t.wrapping_mul(t) + (b - 1) * 8).isqrt() - t) / 2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        Self::max_bottles_drunk(num_bottles, num_exchange)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
