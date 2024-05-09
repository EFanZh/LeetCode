pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn compute_kmp_table(s: &[u8]) -> Box<[u16]> {
        let mut result = vec![0; s.len()].into_boxed_slice();
        let mut matched = 0;
        let mut i = 1;

        while let Some(&c) = s.get(i) {
            loop {
                if c == s[matched] {
                    matched += 1;
                    result[i] = matched as _;

                    break;
                } else if let Some(&next_matched) = result.get(matched.wrapping_sub(1)) {
                    matched = usize::from(next_matched);
                } else {
                    break;
                }
            }

            i += 1;
        }

        result
    }

    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut s = s.into_bytes();
        let part = part.as_bytes();
        let kmp_table = Self::compute_kmp_table(part);
        let mut matched = 0;
        let mut match_history = Vec::<u16>::with_capacity(s.len());
        let mut i = 0;

        while let Some(&c) = s.get(i) {
            loop {
                if c == part[matched] {
                    matched += 1;

                    break;
                } else if let Some(&next_matched) = kmp_table.get(matched.wrapping_sub(1)) {
                    matched = usize::from(next_matched);
                } else {
                    break;
                }
            }

            if matched == part.len() {
                match_history.truncate(match_history.len() - (part.len() - 1));
                matched = usize::from(match_history.last().copied().unwrap_or_default());
            } else {
                s[match_history.len()] = c;
                match_history.push(matched as _);
            }

            i += 1;
        }

        s.truncate(match_history.len());

        String::from_utf8(s).ok().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_occurrences(s: String, part: String) -> String {
        Self::remove_occurrences(s, part)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
