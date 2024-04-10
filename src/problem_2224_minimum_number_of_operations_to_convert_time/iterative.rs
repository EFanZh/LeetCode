pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn parse_time(s: &str) -> u16 {
        let [h0, h1, _, m0, m1]: [_; 5] = s.as_bytes().try_into().ok().unwrap();

        u16::from(h0) * 600 + u16::from(h1) * 60 + u16::from(m0) * 10 + u16::from(m1)
    }

    pub fn convert_time(current: String, correct: String) -> i32 {
        let current = Self::parse_time(&current);
        let correct = Self::parse_time(&correct);
        let mut diff = correct - current;
        let mut result = diff / 60;

        diff %= 60;

        result += diff / 15;
        diff %= 15;

        result += diff / 5;
        diff %= 5;

        result += diff;

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn convert_time(current: String, correct: String) -> i32 {
        Self::convert_time(current, correct)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
