pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

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

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_palindrome(x: i32) -> bool {
        Self::is_palindrome(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
