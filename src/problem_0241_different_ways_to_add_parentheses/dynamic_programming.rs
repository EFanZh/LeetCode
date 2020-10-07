pub struct Solution;

use std::iter::Peekable;
use std::mem;
use std::str::Bytes;

impl Solution {
    fn get_num(iter: &mut Peekable<Bytes>) -> i32 {
        let mut result = i32::from(iter.next().unwrap() - b'0');

        while let Some(&digit @ b'0'..=b'9') = iter.peek() {
            result *= 10;
            result += i32::from(digit - b'0');

            iter.next();
        }

        result
    }

    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        let mut iter = input.bytes().peekable();
        let mut nums = vec![Self::get_num(&mut iter)];
        let mut ops = Vec::new();

        while let Some(op) = iter.next() {
            ops.push(op);
            nums.push(Self::get_num(&mut iter));
        }

        let columns = nums.len();
        let mut cache = vec![Vec::new(); columns * (ops.len() + 1)];

        for (i, &num) in nums.iter().enumerate() {
            cache[i].push(num);
        }

        for length in 1..=ops.len() {
            let (calculated_rows, current_row) = cache.split_at_mut(columns * length);

            for start in 0..columns - length {
                let result = &mut current_row[start];

                for (i, &op) in ops[start..start + length].iter().enumerate() {
                    let left = calculated_rows[columns * i + start].as_slice();
                    let right = calculated_rows[columns * (length - 1 - i) + (start + 1 + i)].as_slice();

                    match op {
                        b'+' => {
                            for &lhs in left {
                                for &rhs in right {
                                    result.push(lhs + rhs);
                                }
                            }
                        }
                        b'-' => {
                            for &lhs in left {
                                for &rhs in right {
                                    result.push(lhs - rhs);
                                }
                            }
                        }
                        _ => {
                            for &lhs in left {
                                for &rhs in right {
                                    result.push(lhs * rhs);
                                }
                            }
                        }
                    }
                }
            }
        }

        mem::take(&mut cache[columns * ops.len()])
    }
}

impl super::Solution for Solution {
    fn diff_ways_to_compute(input: String) -> Vec<i32> {
        Self::diff_ways_to_compute(input)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
