pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(word: &str) -> bool {
        let mut iter = word.bytes();

        match iter.next() {
            Some(b'a'..=b'z') => {}
            Some(b'0'..=b'9' | b'-') => return false,
            _ => return iter.len() == 0,
        }

        while let Some(c) = iter.next() {
            match c {
                b'a'..=b'z' => {}
                b'0'..=b'9' => return false,
                b'-' => {
                    if let Some(b'a'..=b'z') = iter.next() {
                    } else {
                        return false;
                    }

                    while let Some(c) = iter.next() {
                        match c {
                            b'a'..=b'z' => {}
                            b'0'..=b'9' | b'-' => return false,
                            _ => return iter.len() == 0,
                        }
                    }

                    break;
                }
                _ => return iter.len() == 0,
            }
        }

        true
    }

    pub fn count_valid_words(sentence: String) -> i32 {
        sentence.split_whitespace().filter(|&word| Self::check(word)).count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_valid_words(sentence: String) -> i32 {
        Self::count_valid_words(sentence)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
