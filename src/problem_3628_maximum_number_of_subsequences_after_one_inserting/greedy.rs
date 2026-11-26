pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_of_subsequences(s: String) -> i64 {
        let mut right_t = s.bytes().map(|c| u64::from(c == b'T')).sum::<u64>();
        let mut c = 0;
        let mut l = 0;
        let mut ct = 0;
        let mut lc = 0;
        let mut lct = 0;
        let mut max_c_increase = 0;

        for ch in s.bytes() {
            max_c_increase = max_c_increase.max(l * right_t);

            match ch {
                b'C' => {
                    c += 1;
                    lc += l;
                }
                b'L' => l += 1,
                b'T' => {
                    right_t -= 1;
                    ct += c;
                    lct += lc;
                }
                _ => {}
            }
        }

        (lct + ct.max(lc).max(max_c_increase)).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_subsequences(s: String) -> i64 {
        Self::num_of_subsequences(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
