pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_substrings(s: String, c: char) -> i64 {
        let c = c as u8;
        let count = u64::from(s.bytes().fold(0, |sum, x| sum + u32::from(x == c)));

        (count * (count + 1) / 2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_substrings(s: String, c: char) -> i64 {
        Self::count_substrings(s, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
