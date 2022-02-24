pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut result = Vec::with_capacity(s.len() + 1);
        let mut min = 0;
        let mut max = s.len() as _;

        for c in s.bytes() {
            if c == b'D' {
                result.push(max);
                max -= 1;
            } else {
                result.push(min);
                min += 1;
            }
        }

        result.push(min);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn di_string_match(s: String) -> Vec<i32> {
        Self::di_string_match(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
