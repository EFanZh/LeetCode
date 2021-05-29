pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let s = s.into_bytes();
        let mut used = 0_u32;
        let mut unused = [0; 26];
        let mut result_length = 0;

        for &c in &s {
            let count = &mut unused[usize::from(c - b'a')];

            if *count == 0 {
                result_length += 1;
            }

            *count += 1;
        }

        let mut result = Vec::with_capacity(result_length);

        for c in s {
            if used & (1 << (c - b'a')) == 0 {
                while let Some(&last) = result.last() {
                    if c < last && unused[usize::from(last - b'a')] != 0 {
                        result.pop();
                        used ^= 1 << (last - b'a');
                    } else {
                        break;
                    }
                }

                result.push(c);
                used |= 1 << (c - b'a');
            }

            unused[usize::from(c - b'a')] -= 1;
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_duplicate_letters(s: String) -> String {
        Self::remove_duplicate_letters(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
