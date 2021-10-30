pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result = 0_i32;

        while x != 0 {
            if let Some(new_result) = result.checked_mul(10).and_then(|r| r.checked_add(x % 10)) {
                result = new_result;
            } else {
                return 0;
            }

            x /= 10;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse(x: i32) -> i32 {
        Self::reverse(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
