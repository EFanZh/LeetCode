pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut cache = [u32::MAX; 1024];

        cache[0] = 0;

        let mut result = 0;
        let mut state = 0;

        for (i, c) in (1..).zip(s.bytes()) {
            state ^= 1 << (c - b'0');
            state &= (1 << 10) - 1; // Skip bound checking.

            // We only process odd lengths here.

            let mut prev_index = u32::MAX;

            for j in 0..10 {
                prev_index = prev_index.min(cache[state ^ (1 << j)]);
            }

            // The previous index is guaranteed to exist.

            result = result.max(i - prev_index);

            // Update state index.

            let index = &mut cache[state];

            *index = (*index).min(i);
        }

        // Even length case.

        result += u32::from(state == 0);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_awesome(s: String) -> i32 {
        Self::longest_awesome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
