pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut result = 0;
        let mut sum = 0;

        for c in s.bytes() {
            if c == b'L' {
                sum -= 1;
            } else {
                sum += 1;
            }

            if sum == 0 {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn balanced_string_split(s: String) -> i32 {
        Self::balanced_string_split(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
