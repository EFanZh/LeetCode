pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.into_bytes();
        let mut result = s.len();

        for i in 1..s.len() {
            result += s[..i]
                .iter()
                .rev()
                .zip(&s[i..])
                .take_while(|(left, right)| left == right)
                .count();

            result += s[..i]
                .iter()
                .rev()
                .zip(&s[i + 1..])
                .take_while(|(left, right)| left == right)
                .count();
        }

        result as _
    }
}

impl super::Solution for Solution {
    fn count_substrings(s: String) -> i32 {
        Self::count_substrings(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
