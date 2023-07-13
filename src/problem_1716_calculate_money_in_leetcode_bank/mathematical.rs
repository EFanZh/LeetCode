pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let n = n as u32;
        let weeks = n / 7;
        let days = n % 7;

        ((weeks * (7 * weeks + 2 * days + 49) + days * days + days) / 2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn total_money(n: i32) -> i32 {
        Self::total_money(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
