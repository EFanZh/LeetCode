pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    #[allow(clippy::ptr_arg)] // Expected.
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut iter = s.iter_mut();

        while let (Some(left), Some(right)) = (iter.next(), iter.next_back()) {
            mem::swap(left, right);
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_string(s: &mut Vec<char>) {
        Self::reverse_string(s);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
