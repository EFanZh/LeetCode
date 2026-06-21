pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_even(s: String) -> String {
        let mut s = s;
        let keep = s.bytes().rposition(|c| c != b'1').map_or(0, |i| i + 1);

        s.truncate(keep);

        s
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_even(s: String) -> String {
        Self::largest_even(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
