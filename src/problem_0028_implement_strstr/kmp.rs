pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn compute_prefix_function(needle: &[u8]) -> Box<[usize]> {
        let mut prefix_function = vec![0; needle.len()];
        let mut j = 0_usize;

        for (i, c) in needle.iter().copied().enumerate().skip(1) {
            loop {
                if needle[j] == c {
                    j += 1;

                    break;
                } else if let Some(next_j) = prefix_function.get(j.wrapping_sub(1)) {
                    j = *next_j;
                } else {
                    break;
                }
            }

            prefix_function[i] = j;
        }

        prefix_function.into_boxed_slice()
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
