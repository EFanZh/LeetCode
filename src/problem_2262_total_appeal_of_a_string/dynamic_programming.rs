pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let mut prev_indices = [0_u32; 26];
        let mut i = 0;
        let mut result = 0_u64;
        let mut appeal = 0;

        for c in s.into_bytes() {
            i += 1;
            appeal += i - mem::replace(&mut prev_indices[usize::from(c) - usize::from(b'a')], i);
            result += u64::from(appeal);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn appeal_sum(s: String) -> i64 {
        Self::appeal_sum(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
