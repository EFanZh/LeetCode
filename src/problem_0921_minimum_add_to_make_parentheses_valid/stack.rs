pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut result = 0;
        let mut stack = 0;

        for c in s.bytes() {
            if c == b'(' {
                stack += 1;
            } else if stack == 0 {
                result += 1;
            } else {
                stack -= 1;
            }
        }

        result + stack
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_add_to_make_valid(s: String) -> i32 {
        Self::min_add_to_make_valid(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
