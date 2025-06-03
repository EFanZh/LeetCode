pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut s = s.into_bytes();
        let n = s.len();
        let (left, right) = s.split_at_mut(n / 2);

        left.iter_mut().zip(right.iter_mut().rev()).for_each(|(left, right)| {
            if right < left {
                *left = *right;
            } else {
                *right = *left;
            }
        });

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_smallest_palindrome(s: String) -> String {
        Self::make_smallest_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
