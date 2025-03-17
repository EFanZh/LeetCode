pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        if let &[first, second, ref rest @ ..] = sentence.as_bytes() {
            let mut prev_1 = first;
            let mut prev_2 = second;

            for &c in rest {
                if prev_2 == b' ' && prev_1 != c {
                    return false;
                }

                prev_1 = prev_2;
                prev_2 = c;
            }

            first == prev_2
        } else {
            true
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_circular_sentence(sentence: String) -> bool {
        Self::is_circular_sentence(sentence)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
