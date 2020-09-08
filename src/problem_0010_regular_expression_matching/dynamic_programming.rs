pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.into_bytes(), p.into_bytes());
        let columns = p.len() + 1;
        let mut cache = vec![false; columns * (s.len() + 1)];
        let cache_len = cache.len();
        let last_row = &mut cache[cache_len - columns..];

        *last_row.last_mut().unwrap() = true;

        for j in (0..p.len()).rev() {
            if p.get(j + 1) == Some(&b'*') {
                last_row[j] = last_row[j + 2];
            }
        }

        for (i, s_i) in s.iter().enumerate().rev() {
            for (j, p_j) in p.iter().enumerate().rev() {
                cache[columns * i + j] = match (*p_j == b'.' || p_j == s_i, p.get(j + 1) == Some(&b'*')) {
                    (false, false) => continue,
                    (false, true) => cache[columns * i + (j + 2)],
                    (true, false) => cache[columns * (i + 1) + (j + 1)],
                    (true, true) => cache[columns * i + (j + 2)] || cache[columns * (i + 1) + j],
                }
            }
        }

        cache[0]
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
