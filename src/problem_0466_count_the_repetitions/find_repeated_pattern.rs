pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        // https://leetcode.com/problems/count-the-repetitions/discuss/95398/C%2B%2B-solution-inspired-by-%4070664914-with-organized-explanation.

        let n1 = n1 as usize;
        let n2 = n2 as usize;
        let s2 = s2.into_bytes();
        let mut indices = HashMap::new();
        let mut repeat_counts = Vec::with_capacity(n1);
        let mut repeat_count = 0;
        let mut j = 0;

        for k in 0..n1 {
            match indices.entry(j) {
                Entry::Occupied(entry) => {
                    let s1_repeat_start = *entry.get();
                    let prefix_count = repeat_counts[s1_repeat_start];
                    let s1_repeating_blocks = n1 - s1_repeat_start;
                    let s1_pattern_blocks = k - s1_repeat_start;
                    let s2_pattern_blocks = repeat_count - prefix_count;
                    let pattern_count = s1_repeating_blocks / s1_pattern_blocks * s2_pattern_blocks;
                    let extra_s1_blocks = s1_repeating_blocks % s1_pattern_blocks;
                    let suffix_count = repeat_counts[s1_repeat_start + extra_s1_blocks] - prefix_count;

                    repeat_count = prefix_count + pattern_count + suffix_count;

                    break;
                }
                Entry::Vacant(entry) => {
                    repeat_counts.push(repeat_count);
                    entry.insert(k);
                }
            }

            for c in s1.bytes() {
                if c == s2[j] {
                    j += 1;

                    if j == s2.len() {
                        j = 0;
                        repeat_count += 1;
                    }
                }
            }
        }

        (repeat_count / n2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        Self::get_max_repetitions(s1, n1, s2, n2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
