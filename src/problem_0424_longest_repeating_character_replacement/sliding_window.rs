pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.into_bytes();
        let k = k as usize;

        if k + 1 < s.len() {
            let mut letters = [0_u16; 26];

            for &c in &s {
                letters[usize::from(c - b'A')] += 1;
            }

            if usize::from(letters.iter().copied().max().unwrap()) + k < s.len() {
                let mut result = 0;

                for (c, &count) in (b'A'..).zip(&letters) {
                    if count != 0 {
                        let mut start = 0;
                        let mut end = 0;
                        let mut remaining = k;

                        while remaining != 0 {
                            if s[end] != c {
                                remaining -= 1;
                            }

                            end += 1;
                        }

                        result = result.max(end);

                        while let Some(&c_2) = s.get(end) {
                            if c_2 != c {
                                while s[start] == c {
                                    start += 1;
                                }

                                start += 1;
                            }

                            end += 1;

                            result = result.max(end - start);
                        }
                    }
                }

                result as _
            } else {
                s.len() as _
            }
        } else {
            s.len() as _
        }
    }
}

impl super::Solution for Solution {
    fn character_replacement(s: String, k: i32) -> i32 {
        Self::character_replacement(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
