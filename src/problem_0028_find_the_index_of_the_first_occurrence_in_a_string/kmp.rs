pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn compute_prefix_function(s: &[u8]) -> Box<[usize]> {
        let mut result = vec![0; s.len()].into_boxed_slice();
        let mut i = 1;
        let mut matched = 0;

        while let Some(&c) = s.get(i) {
            loop {
                if s[matched] == c {
                    matched += 1;
                    result[i] = matched;

                    break;
                } else if let Some(&next_matched) = result.get(matched.wrapping_sub(1)) {
                    matched = next_matched;
                } else {
                    break;
                }
            }

            i += 1;
        }

        result
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let prefix_function = Self::compute_prefix_function(needle);
        let mut i = 0;
        let mut j = 0;

        'outer: while let Some(mut c_2) = needle.get(j).copied() {
            while let Some(c_1) = haystack.get(i).copied() {
                loop {
                    if c_1 == c_2 {
                        i += 1;
                        j += 1;

                        continue 'outer;
                    } else if let Some(next_j) = prefix_function.get(j.wrapping_sub(1)).copied() {
                        j = next_j;
                        c_2 = needle[j];
                    } else {
                        i += 1;

                        break;
                    }
                }
            }

            return -1;
        }

        (i - j) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        Self::str_str(haystack, needle)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
