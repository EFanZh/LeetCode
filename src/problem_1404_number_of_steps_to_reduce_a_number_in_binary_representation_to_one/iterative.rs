pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut result = 0;
        let mut iter = s.as_bytes()[1..].iter().map(|&c| c == b'0').rev();

        for is_zero in iter.by_ref() {
            if is_zero {
                result += 1;
            } else {
                result += 3;

                for is_zero in iter {
                    result += if is_zero { 2 } else { 1 };
                }

                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_steps(s: String) -> i32 {
        Self::num_steps(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
