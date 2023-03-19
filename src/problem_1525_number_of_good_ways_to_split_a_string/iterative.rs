pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let mut right_counts = [0_u32; 26];
        let mut right = 0;

        for c in s.bytes() {
            let right_count = &mut right_counts[usize::from(c) - usize::from(b'a')];

            right += u32::from(*right_count == 0);
            *right_count += 1;
        }

        let mut left_counts = [0_u32; 26];
        let mut left = 0;
        let mut result = 0;

        for c in s.bytes() {
            let left_count = &mut left_counts[usize::from(c) - usize::from(b'a')];

            left += u32::from(*left_count == 0);
            *left_count += 1;

            let right_count = &mut right_counts[usize::from(c) - usize::from(b'a')];

            *right_count -= 1;
            right -= u32::from(*right_count == 0);

            match left.cmp(&right) {
                Ordering::Less => {}
                Ordering::Equal => result += 1,
                Ordering::Greater => break,
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_splits(s: String) -> i32 {
        Self::num_splits(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
