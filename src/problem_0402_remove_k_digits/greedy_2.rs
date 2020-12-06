pub struct Solution;

use std::cmp::Ordering;
use std::iter;

impl Solution {
    pub fn remove_kdigits(mut num: String, k: i32) -> String {
        let k = k as usize;

        if k != 0 {
            if k == num.len() {
                num.replace_range(.., "0");
            } else {
                num = {
                    let mut num = num.into_bytes();

                    if let Some(stack_base) = num[..k].iter().zip(&num[1..]).position(|(lhs, rhs)| lhs <= rhs) {
                        let result_length = num.len() - k;

                        let stack_length = {
                            let num = &mut num[stack_base..];
                            let k = k - stack_base;

                            let mut stack_top = num[1..result_length]
                                .iter()
                                .zip(&num[2..])
                                .position(|(lhs, rhs)| lhs > rhs)
                                .map_or(result_length, |p| 1 + p);

                            let mut i = stack_top + 1;

                            while i - stack_top != k {
                                let start = i.saturating_sub(k);
                                let digit = num[i];

                                let insertion_point = start
                                    + num[start..stack_top]
                                        .binary_search_by(
                                            |&v| if v > digit { Ordering::Greater } else { Ordering::Less },
                                        )
                                        .unwrap_err();

                                if insertion_point != result_length {
                                    num[insertion_point] = digit;
                                    stack_top = insertion_point + 1;
                                }

                                i += 1;
                            }

                            stack_top
                        };

                        num.copy_within(stack_base..stack_base + stack_length, 0);
                        num.drain(stack_length..stack_length + k);
                    } else {
                        num.drain(..k);
                    }

                    if let Some(i) = num.iter().position(|&c| c != b'0') {
                        num.drain(..i);
                    } else {
                        num.splice(.., iter::once(b'0'));
                    }

                    String::from_utf8(num).unwrap()
                };
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
