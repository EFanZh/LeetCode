pub struct Solution {}

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let mut iter = s.split_whitespace().rev();
        let mut result = String::with_capacity(s.len());

        if let Some(first) = iter.next() {
            result.push_str(first);

            for word in iter {
                result.push(' ');
                result.push_str(word);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn shortest_palindrome(s: String) -> String {
        Self::shortest_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
