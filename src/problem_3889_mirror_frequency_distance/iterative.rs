pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn mirror_frequency(s: String) -> i32 {
        let mut counts = [0; 18];

        for c in s.into_bytes() {
            match c {
                b'0'..b'5' => counts[usize::from(c) - usize::from(b'0')] += 1,
                b'5'..=b'9' => counts[usize::from(b'9') - usize::from(c)] -= 1,
                b'a'..b'n' => counts[5 + usize::from(c) - usize::from(b'a')] += 1,
                _ => counts[5 + usize::from(b'z') - usize::from(c)] -= 1,
            }
        }

        counts.iter().copied().map(i32::abs).sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn mirror_frequency(s: String) -> i32 {
        Self::mirror_frequency(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
