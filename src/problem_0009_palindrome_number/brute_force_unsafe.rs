pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else {
            let mut x2 = x;
            let mut reversed = 0_i32;

            while x2 != 0 {
                reversed = reversed.wrapping_mul(10).wrapping_add(x2 % 10);
                x2 /= 10;
            }

            x == reversed
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
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
