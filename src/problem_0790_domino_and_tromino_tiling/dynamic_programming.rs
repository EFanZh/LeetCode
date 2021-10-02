pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let modulus = 1_000_000_007;
        let mut full_1 = 1_u32;
        let mut full_2 = 1_u32;
        let mut half_2 = 0_u32;

        for _ in 2..=n {
            let full_3 = (full_1 + full_2 + half_2 * 2) % modulus;
            let half_3 = (full_1 + half_2) % modulus;

            full_1 = full_2;
            full_2 = full_3;
            half_2 = half_3;
        }

        full_2 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_tilings(n: i32) -> i32 {
        Self::num_tilings(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
