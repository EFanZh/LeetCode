pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        const INDICES: [u8; 26] = [
            0, 5, 6, 7, 1, 8, 9, 10, 2, 11, 12, 13, 14, 15, 3, 16, 17, 18, 19, 20, 4, 21, 22, 23, 24, 25,
        ];

        let mut counts = [0_u8; 26];

        for c in s.into_bytes() {
            counts[usize::from(INDICES[usize::from(c) - usize::from(b'a')])] += 1;
        }

        let (vowels, consonant) = counts.split_at(5);

        i32::from(vowels.iter().copied().max().unwrap() + consonant.iter().copied().max().unwrap())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_freq_sum(s: String) -> i32 {
        Self::max_freq_sum(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
