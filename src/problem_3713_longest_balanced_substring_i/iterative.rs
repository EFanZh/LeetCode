pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let mut result = (s.len() as u32).min(2);
        let mut iter = s.bytes();

        while let Some(c) = iter.next() {
            let mut length = 1;
            let mut counts = [0_u16; 26];
            let mut max_count = 1;
            let mut max_count_count = 1;
            let mut non_zero_count = 1;

            counts[usize::from(c) - usize::from(b'a')] = 1;

            for c in iter.clone() {
                length += 1;

                let count = &mut counts[usize::from(c) - usize::from(b'a')];

                non_zero_count += u32::from(*count == 0);
                *count += 1;

                let count = *count;

                match count.cmp(&max_count) {
                    Ordering::Less => {}
                    Ordering::Equal => max_count_count += 1,
                    Ordering::Greater => {
                        max_count = count;
                        max_count_count = 1;
                    }
                }

                if non_zero_count == max_count_count {
                    result = result.max(length);
                }
            }
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_balanced(s: String) -> i32 {
        Self::longest_balanced(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
