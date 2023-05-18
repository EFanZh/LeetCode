pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut counts = [0_u32; 26];
        let mut max_count = 0;

        for &c in s {
            let count = &mut counts[usize::from(c) - usize::from(b'a')];

            *count += 1;

            max_count = max_count.max(*count);
        }

        let mut frequencies = vec![0_u32; n].into_boxed_slice();

        for count in counts {
            if let Some(frequency) = frequencies.get_mut((count as usize).wrapping_sub(1)) {
                *frequency += 1;
            }
        }

        let mut result = 0;
        let mut right = 0_u32;

        for &left in frequencies[..max_count as usize].iter().rev() {
            let extra = right.saturating_sub(1);

            result += extra;
            right = left + extra;
        }

        if right > 1 {
            result += right - 1;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_deletions(s: String) -> i32 {
        Self::min_deletions(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
