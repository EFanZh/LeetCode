pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        const fn c2(x: u64) -> u64 {
            x * x.wrapping_sub(1) / 2
        }

        let n = u64::from(n.cast_unsigned());
        let limit = u64::from(limit.cast_unsigned());
        let mut x = n + 2;
        let mut result = c2(x);

        x = x.saturating_sub(limit + 1);
        result -= 3 * c2(x);

        x = x.saturating_sub(limit + 1);
        result += 3 * c2(x);

        x = x.saturating_sub(limit + 1);
        result -= c2(x);

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distribute_candies(n: i32, limit: i32) -> i64 {
        Self::distribute_candies(n, limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
