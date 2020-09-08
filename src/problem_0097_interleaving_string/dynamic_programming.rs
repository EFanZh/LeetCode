pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() == s3.len() {
            let s3 = s3.into_bytes();
            let mut cache = vec![false; s2.len() + 1];

            cache[0] = true;

            for (j, s2_j) in s2.bytes().enumerate() {
                if s3[j] == s2_j {
                    cache[j + 1] = true;
                } else {
                    break;
                };
            }

            for (i, s1_i) in s1.into_bytes().into_iter().enumerate() {
                if s3[i] != s1_i {
                    cache[0] = false;
                }

                for (j, s2_j) in s2.bytes().enumerate() {
                    let c = s3[i + j + 1];

                    if c == s1_i {
                        if c == s2_j {
                            cache[j + 1] |= cache[j];
                        }
                    } else {
                        cache[j + 1] = c == s2_j && cache[j];
                    }
                }
            }

            cache[s2.len()]
        } else {
            false
        }
    }
}

impl super::Solution for Solution {
    fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        Self::is_interleave(s1, s2, s3)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
