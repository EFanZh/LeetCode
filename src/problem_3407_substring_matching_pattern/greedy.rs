pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        let (left, right) = p.split_once('*').unwrap();

        s.find(left).is_some_and(|i| s[i + left.len()..].contains(right))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_match(s: String, p: String) -> bool {
        Self::has_match(s, p)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
