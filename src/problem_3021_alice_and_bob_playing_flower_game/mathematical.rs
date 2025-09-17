pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let n = u64::from(n.cast_unsigned());
        let m = u64::from(m.cast_unsigned());
        let even_1 = n / 2;
        let odd_1 = n - even_1;
        let even_2 = m / 2;
        let odd_2 = m - even_2;

        (odd_1 * even_2 + even_1 * odd_2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn flower_game(n: i32, m: i32) -> i64 {
        Self::flower_game(n, m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
