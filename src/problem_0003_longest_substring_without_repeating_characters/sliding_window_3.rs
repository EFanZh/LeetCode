pub struct Solution {}

use std::mem;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        const INVALID_INDEX: isize = -1;
        let s = s.into_bytes();
        let mut result = 0;
        let mut start = 0;
        let mut byte_locations = [INVALID_INDEX; 256];

        for (i, byte) in s.into_iter().enumerate() {
            let i = i as isize;
            let old_index = mem::replace(&mut byte_locations[byte as usize], i);

            if old_index < start {
                result = result.max(i - start + 1);
            } else {
                start = old_index + 1;
            }
        }

        result as _
    }
}

impl super::Solution for Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        Self::length_of_longest_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
