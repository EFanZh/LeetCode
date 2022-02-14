pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(words: Vec<String>) -> i32 {
        words
            .into_iter()
            .map(|word| {
                let mut word = word.into_bytes();
                let mut i = 0;
                let mut c = word[i];

                while let b'a'..=b'z' = c {
                    i += 1;
                    c = word[i];
                }

                let mut retained = i;

                if c == b'.' {
                    loop {
                        i += 1;
                        c = word[i];

                        match c {
                            b'a'..=b'z' => {
                                word[retained] = c;
                                retained += 1;
                            }
                            b'.' => {}
                            _ => break,
                        }
                    }
                }

                if c == b'+' {
                    loop {
                        i += 1;

                        if word[i] == b'@' {
                            break;
                        }
                    }
                }

                word.splice(retained..i, None);

                word
            })
            .collect::<HashSet<_>>()
            .len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_unique_emails(emails: Vec<String>) -> i32 {
        Self::num_unique_emails(emails)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
