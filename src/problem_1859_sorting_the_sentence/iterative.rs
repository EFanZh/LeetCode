pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words = [""; 9];

        for word in s.split(' ') {
            let cut = word.len() - 1;
            let index = usize::from(word.as_bytes()[cut]) - usize::from(b'0') - 1;
            let word = &word[..cut];

            words[index] = word;
        }

        let mut result = String::with_capacity(s.len());

        result.push_str(words[0]);

        for &word in &words[1..] {
            if word.is_empty() {
                break;
            }

            result.push(' ');
            result.push_str(word);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_sentence(s: String) -> String {
        Self::sort_sentence(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
