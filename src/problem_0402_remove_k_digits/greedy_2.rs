pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn remove_kdigits(mut num: String, k: i32) -> String {
        let n = num.len();
        let k = k as usize;

        if k == n {
            num.replace_range(.., "0");
        } else if let Some(stack_base) = num.as_bytes()[..k]
            .iter()
            .zip(&num.as_bytes()[1..])
            .position(|(lhs, rhs)| lhs <= rhs)
        {
            let mut bytes = num.into_bytes();
            let length = n - k;
            let stack_top_end = stack_base + length;

            let mut stack_top = bytes[stack_base + 1..stack_top_end]
                .iter()
                .zip(&bytes[stack_base + 2..])
                .position(|(lhs, rhs)| lhs > rhs)
                .map_or(stack_top_end, |p| stack_base + 1 + p);

            let mut i = stack_top + 1;
            let to_remove = k - stack_base;

            while let Some(&digit) = bytes.get(i) {
                let start = stack_base + (i - stack_base).saturating_sub(to_remove);

                let insertion_point = start
                    + bytes[start..stack_top]
                        .binary_search_by(|&d| if d <= digit { Ordering::Less } else { Ordering::Greater })
                        .unwrap_err();

                if i - insertion_point == to_remove {
                    stack_top = insertion_point;

                    break;
                } else if insertion_point != stack_top_end {
                    bytes[insertion_point] = digit;
                    stack_top = insertion_point + 1;
                }

                i += 1;
            }

            bytes.copy_within(stack_base..stack_top, 0);
            bytes.drain(stack_top - stack_base..i);

            if let Some(i) = bytes.iter().position(|&d| d != b'0') {
                bytes.drain(..i);
            } else {
                bytes.truncate(1);
            }

            num = String::from_utf8(bytes).unwrap();
        } else if let Some(i) = num.as_bytes()[k..].iter().position(|&d| d != b'0') {
            num.drain(..k + i);
        } else {
            num.replace_range(.., "0");
        }

        num
    }
}

impl super::Solution for Solution {
    fn remove_kdigits(num: String, k: i32) -> String {
        Self::remove_kdigits(num, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
