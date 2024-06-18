pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut counts = [0_u32; 676];
        let mut result = 0_u32;
        let mut unpaired = 0_u32;

        for word in words {
            let [c0, c1]: [_; 2] = word.into_bytes().try_into().ok().unwrap();
            let (c0, c1) = (u16::from(c0 - b'a'), u16::from(c1 - b'a'));

            if c0 == c1 {
                let count = &mut counts[usize::from(26 * c0 + c1)];

                if *count == 0 {
                    unpaired += 1;
                    *count += 1;
                } else {
                    unpaired -= 1;
                    *count -= 1;
                    result += 4;
                }
            } else {
                let other_count = &mut counts[usize::from(26 * c1 + c0)];

                if *other_count == 0 {
                    counts[usize::from(26 * c0 + c1)] += 1;
                } else {
                    *other_count -= 1;
                    result += 4;
                }
            }
        }

        if unpaired != 0 {
            result += 2;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_palindrome(words: Vec<String>) -> i32 {
        Self::longest_palindrome(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
