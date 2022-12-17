pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let n = n as u64;
        let mut result = 1;

        for i in 1..=n {
            result = (result * (i * 2 - 1) * i) % 1_000_000_007;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_orders(n: i32) -> i32 {
        Self::count_orders(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
