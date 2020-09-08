pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else {
            let mut x2 = x;
            let mut reversed = 0_i32;

            while x2 != 0 {
                if let Some(new_reversed) = reversed.checked_mul(10).and_then(|r| r.checked_add(x2 % 10)) {
                    reversed = new_reversed;
                } else {
                    return false;
                }

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
