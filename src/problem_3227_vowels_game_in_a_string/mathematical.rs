pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.bytes().any(|c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u'))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn does_alice_win(s: String) -> bool {
        Self::does_alice_win(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
