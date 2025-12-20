pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut counts = [0_u32; 26];

        for c in s.into_bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        counts
            .iter()
            .map(|&count| if count < 3 { count } else { 2 - count % 2 })
            .sum::<u32>()
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_length(s: String) -> i32 {
        Self::minimum_length(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
