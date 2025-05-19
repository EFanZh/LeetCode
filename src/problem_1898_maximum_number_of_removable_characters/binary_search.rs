pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn is_subsequence(s: &[u8], p: &[u8]) -> bool {
        let mut iter = s.iter().copied();

        for &expected in p {
            if iter.all(|c| c != expected) {
                return false;
            }
        }

        true
    }

    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let mut s = s.into_bytes();
        let p = p.as_bytes();
        let mut saved = Vec::with_capacity(removable.len());
        let mut left = 1;
        let mut right = removable.len() + 1;

        while left < right {
            let middle = usize::midpoint(left, right);
            let indices = removable[..middle].iter().map(|&i| i as u32 as usize);

            saved.extend(indices.clone().map(|i| mem::take(&mut s[i])));

            if Self::is_subsequence(&s, p) {
                left = middle + 1;
            } else {
                right = middle;
            }

            for (i, &c) in indices.zip(&saved) {
                s[i] = c;
            }

            saved.clear();
        }

        (left - 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        Self::maximum_removals(s, p, removable)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
