pub struct Solution {}

impl Solution {
    fn is_segment_match(s: &[u8], p: &[u8]) -> bool {
        for (s_char, p_char) in s.iter().zip(p) {
            if s_char != p_char && *p_char != b'?' {
                return false;
            }
        }

        true
    }

    fn find_chunk<'a>(s: &'a [u8], p: &[u8]) -> Option<&'a [u8]> {
        if p.is_empty() {
            Some(s)
        } else {
            s.windows(p.len()).enumerate().find_map(|(i, chunk)| {
                if Self::is_segment_match(chunk, p) {
                    Some(&s[i + p.len()..])
                } else {
                    None
                }
            })
        }
    }

    pub fn is_match(s: String, p: String) -> bool {
        let mut patterns = p.as_bytes().split(|c| *c == b'*');
        let first_pattern = patterns.next().unwrap();

        if first_pattern.len() <= s.len() {
            let (first_str, s) = s.as_bytes().split_at(first_pattern.len());

            if Self::is_segment_match(first_str, first_pattern) {
                if let Some(last_pattern) = patterns.next_back() {
                    if let Some(rest_len) = s.len().checked_sub(last_pattern.len()) {
                        let (mut s, last_str) = s.split_at(rest_len);

                        if Self::is_segment_match(last_str, last_pattern) {
                            for pattern in patterns {
                                if let Some(new_rest_str) = Self::find_chunk(s, pattern) {
                                    s = new_rest_str;
                                } else {
                                    return false;
                                }
                            }

                            return true;
                        }
                    }
                } else {
                    return s.is_empty();
                }
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn is_match(s: String, p: String) -> bool {
        Self::is_match(s, p)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
