pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let mut states = [(0_u32, 0_u32), (0_u32, 0_u32)];

        s.into_bytes().into_iter().fold(0_u64, |result, c| {
            let c = usize::from(c & 1);

            states[c].0 += 1;
            states[c].1 += states[1 - c].0;

            result + u64::from(states[1 - c].1)
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
