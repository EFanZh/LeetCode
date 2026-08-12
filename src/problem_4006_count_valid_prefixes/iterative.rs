pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_valid_prefixes(s: String) -> i32 {
        let mut zero_count = 0_u32;
        let mut one_count = 0_u32;
        let mut result = 0;

        for c in s.bytes() {
            if c == b'0' {
                zero_count += 1;
            } else {
                one_count += 1;
            }

            result += i32::from(zero_count.abs_diff(one_count) < 2);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_valid_prefixes(s: String) -> i32 {
        Self::count_valid_prefixes(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
