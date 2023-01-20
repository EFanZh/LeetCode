pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        const MODULUS: u64 = 1_000_000_007;

        let mut state = (6_u64, 6_u64);

        for _ in 1..n {
            state = (
                (state.0 * 3 + state.1 * 2) % MODULUS,
                (state.0 * 2 + state.1 * 2) % MODULUS,
            );
        }

        ((state.0 + state.1) % MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_ways(n: i32) -> i32 {
        Self::num_of_ways(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
