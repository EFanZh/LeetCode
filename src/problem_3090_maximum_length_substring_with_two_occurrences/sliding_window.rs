pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut start = 0;
        let mut counts = [0_u8; 26];
        let mut count_over_2 = 0;

        for &c in s {
            let count = &mut counts[usize::from(c) - usize::from(b'a')];

            count_over_2 += u8::from(*count == 2);
            *count += 1;

            if count_over_2 != 0 {
                let start_count = &mut counts[usize::from(s[start]) - usize::from(b'a')];

                *start_count -= 1;
                count_over_2 -= u8::from(*start_count == 2);
                start += 1;
            }
        }

        (s.len() - start) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_length_substring(s: String) -> i32 {
        Self::maximum_length_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
