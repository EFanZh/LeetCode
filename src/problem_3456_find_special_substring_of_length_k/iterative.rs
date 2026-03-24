pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let mut prev = 0;
        let mut length = 0;

        for c in s.bytes() {
            if c == prev {
                length += 1;
            } else if length == k {
                return true;
            } else {
                prev = c;
                length = 1;
            }
        }

        length == k
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_special_substring(s: String, k: i32) -> bool {
        Self::has_special_substring(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
