pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let mut s = s;
        let s_bytes = s.as_bytes();
        let mut cache = vec![0_u32; s_bytes.len()];
        let mut matched = 0;
        let mut i = 1;

        for &c in &s_bytes[1..] {
            loop {
                if s_bytes[matched as usize] == c {
                    matched += 1;

                    break;
                } else if let Some(&next_matched) = cache.get((matched as usize).wrapping_sub(1)) {
                    matched = next_matched;
                } else {
                    break;
                }
            }

            cache[i] = matched;
            i += 1;
        }

        s.truncate(*cache.last().unwrap() as _);

        s
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_prefix(s: String) -> String {
        Self::longest_prefix(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
