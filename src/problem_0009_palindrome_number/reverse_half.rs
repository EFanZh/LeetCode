pub struct Solution {}

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            false
        } else if x == 0 {
            true
        } else if x % 10 == 0 {
            false
        } else {
            let mut reversed = 0;

            while reversed < x {
                reversed *= 10;
                reversed += x % 10;
                x /= 10;
            }

            reversed == x || reversed / 10 == x
        }
    }
}

impl super::Solution for Solution {
    fn is_palindrome(x: i32) -> bool {
        Self::is_palindrome(x)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
