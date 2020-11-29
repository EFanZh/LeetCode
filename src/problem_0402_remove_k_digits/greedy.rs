pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn remove_kdigits(mut num: String, k: i32) -> String {
        if k != 0 {
            let k = k as usize;
            let n = num.len();

            if k < n {
                let mut bytes = num.into_bytes();
                let result_length = n - k;

                let mut stack_length = bytes[..result_length]
                    .iter()
                    .zip(&bytes[1..])
                    .position(|(lhs, rhs)| lhs > rhs)
                    .unwrap_or(result_length);

                let mut i = stack_length + 1;

                while i - stack_length != k {
                    let start = i.saturating_sub(k);
                    let digit = bytes[i];

                    let insertion_point = start
                        + bytes[start..stack_length]
                            .binary_search_by(|&v| if v > digit { Ordering::Greater } else { Ordering::Less })
                            .unwrap_err();

                    if insertion_point != result_length {
                        bytes[insertion_point] = digit;
                        stack_length = insertion_point + 1;
                    }

                    i += 1;
                }

                bytes.drain(stack_length..i);

                if let Some(i) = bytes.iter().position(|&d| d != b'0') {
                    bytes.drain(..i);
                } else {
                    bytes.truncate(1);
                }

                num = String::from_utf8(bytes).unwrap();
            } else {
                num.replace_range(.., "0");
            }
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
