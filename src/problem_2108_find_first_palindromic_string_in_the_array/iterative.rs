pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|word| {
                let mut iter = word.bytes();

                while let Some(lhs) = iter.next() {
                    if let Some(rhs) = iter.next_back() {
                        if lhs != rhs {
                            return false;
                        }
                    } else {
                        break;
                    }
                }

                true
            })
            .unwrap_or_default()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn first_palindrome(words: Vec<String>) -> String {
        Self::first_palindrome(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
