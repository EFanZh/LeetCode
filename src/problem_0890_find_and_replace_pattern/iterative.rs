pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut words = words;

        words.retain(|word| {
            let mut mappings = [0; 26];
            let mut used = 0;

            for (c, p) in word.bytes().zip(pattern.bytes()) {
                let mapping = &mut mappings[usize::from(c) - usize::from(b'a')];

                if *mapping == 0 {
                    let probe = 1 << (p - b'a');

                    if used & probe == 0 {
                        *mapping = p;
                        used |= probe;
                    } else {
                        return false;
                    }
                } else if *mapping != p {
                    return false;
                }
            }

            true
        });

        words
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        Self::find_and_replace_pattern(words, pattern)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
