pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut counts = [0_u8; 26];

        for c in license_plate.into_bytes() {
            match c {
                b'A'..=b'Z' => counts[usize::from(c - b'A')] += 1,
                b'a'..=b'z' => counts[usize::from(c - b'a')] += 1,
                _ => {}
            }
        }

        let total_letters = usize::from(counts.iter().sum::<u8>());
        let expected_matches = counts.iter().filter(|&&count| count != 0).count();
        let mut result = None::<String>;

        for word in words {
            if word.len() >= total_letters && result.as_deref().map_or(true, |result| word.len() < result.len()) {
                let mut word_counts = [0; 26];
                let mut current_matches = 0;

                for c in word.bytes() {
                    let index = usize::from(c - b'a');
                    let slot = &mut word_counts[index];

                    *slot += 1;

                    if *slot == counts[index] {
                        current_matches += 1;

                        if current_matches == expected_matches {
                            result = Some(word);

                            break;
                        }
                    }
                }
            }
        }

        result.unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        Self::shortest_completing_word(license_plate, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
