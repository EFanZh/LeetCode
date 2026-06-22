pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn residue_prefixes(s: String) -> i32 {
        let mut seen = 0_u32;
        let mut unique_count = 0;
        let mut expected = 0;
        let mut result = 0;

        for c in s.bytes() {
            expected = if expected == 2 { 0 } else { expected + 1 };

            let probe = 1 << (c - b'a');

            if seen & probe == 0 {
                if unique_count == 2 {
                    break;
                }

                seen |= probe;
                unique_count += 1;
            }

            if unique_count == expected {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn residue_prefixes(s: String) -> i32 {
        Self::residue_prefixes(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
