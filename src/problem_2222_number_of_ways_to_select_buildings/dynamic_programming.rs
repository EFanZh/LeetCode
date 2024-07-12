pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let mut zero = 0_u32;
        let mut one_zero = 0_u32;
        let mut one = 0_u32;
        let mut zero_one = 0_u32;

        s.into_bytes().into_iter().fold(0_u64, |result, c| {
            result
                + u64::from(if c == b'0' {
                    zero += 1;
                    one_zero += one;

                    zero_one
                } else {
                    one += 1;
                    zero_one += zero;

                    one_zero
                })
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_ways(s: String) -> i64 {
        Self::number_of_ways(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
