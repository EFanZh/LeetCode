pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut s = s.into_bytes();
        let mut iter = s.iter_mut().filter(|&&mut c| c.is_ascii_alphabetic());

        while let Some(left) = iter.next() {
            if let Some(right) = iter.next_back() {
                mem::swap(left, right);
            } else {
                break;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_only_letters(s: String) -> String {
        Self::reverse_only_letters(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
