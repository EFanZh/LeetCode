pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut s = s.into_bytes();
        let mut right_counts = [0_u32; 26];

        for c in &mut s {
            *c -= b'a';

            right_counts[usize::from(*c)] += 1;
        }

        let mut left_counts = [0_u32; 26];
        let mut states = [0_u32; 26];

        for c in s {
            let c = usize::from(c);

            right_counts[c] -= 1;

            let state = &mut states[c];

            for (i, (&left_count, &right_count)) in (0..).zip(left_counts.iter().zip(&right_counts)) {
                if left_count != 0 && right_count != 0 {
                    *state |= 1 << i;
                }
            }

            left_counts[c] += 1;
        }

        states.iter().map(|state| state.count_ones()).sum::<u32>() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_palindromic_subsequence(s: String) -> i32 {
        Self::count_palindromic_subsequence(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
