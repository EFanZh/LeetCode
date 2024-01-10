pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(source: &str, target: &str) -> bool {
        let source = source.as_bytes();
        let target = target.as_bytes();
        let mut word_start = 0;
        let mut iter = (1..).zip(source.iter().zip(target));

        loop {
            if let Some((i, (&lhs, &rhs))) = iter.next() {
                if lhs != rhs {
                    break;
                }

                if lhs == b' ' {
                    word_start = i;
                }
            } else {
                if target.get(source.len()).map_or(true, |&c| c == b' ') {
                    return true;
                }

                break;
            }
        }

        let split = target.len() - (source.len() - word_start);

        target.get(split.wrapping_sub(1)).map_or(true, |&c| c == b' ') && source[word_start..] == target[split..]
    }

    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        if sentence2.len() < sentence1.len() {
            Self::check(&sentence2, &sentence1)
        } else {
            Self::check(&sentence1, &sentence2)
                || (sentence1.len() == sentence2.len() && Self::check(&sentence2, &sentence1))
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        Self::are_sentences_similar(sentence1, sentence2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
