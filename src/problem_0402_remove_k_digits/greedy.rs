pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn remove_kdigits(mut num: String, k: i32) -> String {
        let n = num.len();
        let k = k as usize;

        if k == n {
            num.replace_range(.., "0");
        } else {
            let mut bytes = num.into_bytes();
            let length = n - k;

            let mut stack_top = bytes[..length]
                .iter()
                .zip(&bytes[1..])
                .position(|(lhs, rhs)| lhs > rhs)
                .unwrap_or(length);

            let mut i = stack_top + 1;

            while i - stack_top != k {
                let start = i.saturating_sub(k);
                let digit = bytes[i];

                let insertion_point = start
                    + bytes[start..stack_top]
                        .binary_search_by(|&d| if d <= digit { Ordering::Less } else { Ordering::Greater })
                        .unwrap_err();

                if insertion_point != length {
                    bytes[insertion_point] = digit;
                    stack_top = insertion_point + 1;
                }

                i += 1;
            }

            bytes.drain(stack_top..i);

            if let Some(i) = bytes.iter().position(|&d| d != b'0') {
                bytes.drain(..i);
            } else {
                bytes.truncate(1);
            }

            num = String::from_utf8(bytes).unwrap();
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
