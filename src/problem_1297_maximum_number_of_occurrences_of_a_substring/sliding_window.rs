pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let _ = max_size;
        let max_letters = max_letters as u32;
        let min_size = min_size as _;
        let (left, right) = s.as_bytes().split_at(min_size);
        let mut counts = [0_u8; 26];
        let mut unique = 0;

        for &c in left {
            let count = &mut counts[usize::from(c) - usize::from(b'a')];

            if *count == 0 {
                unique += 1;
            }

            *count += 1;
        }

        let mut windows = HashMap::new();

        if unique <= max_letters {
            windows.insert(left, 1_u32);
        }

        for ((old, &new), window) in s.bytes().zip(right).zip(s.as_bytes()[1..].windows(min_size)) {
            let old_count = &mut counts[usize::from(old) - usize::from(b'a')];

            *old_count -= 1;

            if *old_count == 0 {
                unique -= 1;
            }

            let new_count = &mut counts[usize::from(new) - usize::from(b'a')];

            if *new_count == 0 {
                unique += 1;
            }

            *new_count += 1;

            if unique <= max_letters {
                windows.entry(window).and_modify(|count| *count += 1).or_insert(1);
            }
        }

        windows.into_values().max().unwrap_or(0) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        Self::max_freq(s, max_letters, min_size, max_size)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
