pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut counts = [0_u16; 26];

        for c in s.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut iter = counts.iter().copied().filter(|&count| count != 0);
        let first = iter.next().unwrap();

        iter.all(|c| c == first)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn are_occurrences_equal(s: String) -> bool {
        Self::are_occurrences_equal(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
