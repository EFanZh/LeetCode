pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_palindrome(mut iter: impl DoubleEndedIterator<Item = impl Eq>) -> bool {
        while let (Some(left), Some(right)) = (iter.next(), iter.next_back()) {
            if left != right {
                return false;
            }
        }

        true
    }

    pub fn valid_palindrome(s: String) -> bool {
        let mut iter = s.bytes();

        while let (Some(left), Some(right)) = (iter.next(), iter.next_back()) {
            if left != right {
                let mut iter_2 = iter.clone();

                if let Some(left_2) = iter.next() {
                    return (left_2 == right && Self::is_palindrome(iter))
                        || (iter_2.next_back() == Some(left) && Self::is_palindrome(iter_2));
                }

                break;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_palindrome(s: String) -> bool {
        Self::valid_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
