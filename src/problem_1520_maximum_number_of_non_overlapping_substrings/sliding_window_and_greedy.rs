pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        const N: usize = 26;

        // Calculate character ranges.

        let mut ranges = [(usize::MAX, usize::MAX); N];
        let mut characters = 0_u32;

        for (i, c) in (0..).zip(s.bytes()) {
            let range = &mut ranges[usize::from(c) - usize::from(b'a')];

            if range.0 == usize::MAX {
                range.0 = i;
                characters |= 1 << (c - b'a');
            }

            range.1 = i;
        }

        // Find intervals.

        let mut intervals = [(0, 0); N];
        let mut intervals_length = 0;

        while characters != 0 {
            let mut range = ranges[characters.trailing_zeros() as usize];
            let mut i = range.0 + 1;

            loop {
                if i < range.1 {
                    let (new_start, new_end) = ranges[usize::from(s.as_bytes()[i]) - usize::from(b'a')];

                    if new_start < range.0 {
                        break;
                    }

                    range.1 = range.1.max(new_end);
                } else {
                    intervals[intervals_length] = range;
                    intervals_length += 1;

                    break;
                }

                i += 1;
            }

            characters &= characters - 1;
        }

        // Greedy search.

        let intervals = &mut intervals[..intervals_length];

        intervals.sort_unstable_by_key(|&(_, end)| end);

        let mut prev_end = 0;
        let mut result = Vec::with_capacity(intervals_length);

        for &(start, end) in &*intervals {
            if start >= prev_end {
                prev_end = end + 1;

                result.push(s[start..prev_end].to_string());
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_num_of_substrings(s: String) -> Vec<String> {
        Self::max_num_of_substrings(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
