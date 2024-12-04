pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut counts = [false; 26];
        let mut result = 0;

        for c in s.bytes() {
            match &mut counts[usize::from(c) - usize::from(b'a')] {
                state @ false => *state = true,
                true => {
                    result = c;

                    break;
                }
            }
        }

        char::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn repeated_character(s: String) -> char {
        Self::repeated_character(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
