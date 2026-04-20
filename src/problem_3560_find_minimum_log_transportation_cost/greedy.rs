pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        let max = n.cast_unsigned().max(m.cast_unsigned());
        let k = k.cast_unsigned();

        (if max > k { u64::from(max - k) * u64::from(k) } else { 0 }).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        Self::min_cutting_cost(n, m, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
