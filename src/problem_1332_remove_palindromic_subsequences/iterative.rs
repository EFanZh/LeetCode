pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_palindrome(mut iter: impl DoubleEndedIterator<Item = impl Eq>) -> bool {
        while let Some(left) = iter.next() {
            if let Some(right) = iter.next_back() {
                if left != right {
                    return false;
                }
            } else {
                break;
            }
        }

        true
    }

    pub fn remove_palindrome_sub(s: String) -> i32 {
        if Self::is_palindrome(s.bytes()) {
            1
        } else {
            2
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_palindrome_sub(s: String) -> i32 {
        Self::remove_palindrome_sub(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
