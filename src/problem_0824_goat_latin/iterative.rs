pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut result = String::with_capacity(sentence.len() * 2);

        let mut iter = sentence.split_ascii_whitespace().enumerate().map(|(i, word)| {
            move |result: &mut String| {
                match word.as_bytes()[0] {
                    b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u' => result.push_str(word),
                    c => {
                        result.push_str(&word[1..]);
                        result.push(char::from(c));
                    }
                }

                result.push_str("maa");

                for _ in 0..i {
                    result.push('a');
                }
            }
        });

        iter.next().unwrap()(&mut result);

        for f in iter {
            result.push(' ');
            f(&mut result);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn to_goat_latin(sentence: String) -> String {
        Self::to_goat_latin(sentence)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
