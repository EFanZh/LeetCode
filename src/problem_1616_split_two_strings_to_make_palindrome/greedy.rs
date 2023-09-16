pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_palindrome(mut iter: impl DoubleEndedIterator<Item = u8>) -> bool {
        while let Some(first) = iter.next() {
            if let Some(last) = iter.next_back() {
                if first != last {
                    return false;
                }
            } else {
                break;
            }
        }

        true
    }

    fn check(a: &[u8], b: &[u8]) -> bool {
        let mut iter = a.iter().copied().zip(b.iter().copied());

        while let Some((left_top, left_bottom)) = iter.next() {
            if let Some((right_top, right_bottom)) = iter.next_back() {
                if left_top != right_bottom {
                    return (left_top == right_top && Self::is_palindrome(iter.clone().map(|(top, _)| top)))
                        || (left_bottom == right_bottom && Self::is_palindrome(iter.map(|(_, bottom)| bottom)));
                }
            } else {
                break;
            }
        }

        true
    }

    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();

        Self::check(a, b) || Self::check(b, a)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_palindrome_formation(a: String, b: String) -> bool {
        Self::check_palindrome_formation(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
