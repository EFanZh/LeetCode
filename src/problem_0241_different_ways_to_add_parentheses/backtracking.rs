pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter::Peekable;
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

    fn helper(nums: &[i32], ops: &[u8], callback: &mut dyn FnMut(i32)) {
        if ops.is_empty() {
            callback(nums[0]);
        } else {
            for (i, &op) in ops.iter().enumerate() {
                let (left_nums, right_nums) = nums.split_at(i + 1);
                let (left_ops, right_ops) = (&ops[..i], &ops[i + 1..]);

                match op {
                    b'+' => Self::helper(left_nums, left_ops, &mut |lhs| {
                        Self::helper(right_nums, right_ops, &mut |rhs| callback(lhs + rhs));
                    }),
                    b'-' => Self::helper(left_nums, left_ops, &mut |lhs| {
                        Self::helper(right_nums, right_ops, &mut |rhs| callback(lhs - rhs));
                    }),
                    _ => Self::helper(left_nums, left_ops, &mut |lhs| {
                        Self::helper(right_nums, right_ops, &mut |rhs| callback(lhs * rhs));
                    }),
                }
            }
        }
    }

    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        let mut iter = input.bytes().peekable();
        let mut nums = vec![Self::get_num(&mut iter)];
        let mut ops = Vec::new();

        while let Some(op) = iter.next() {
            ops.push(op);
            nums.push(Self::get_num(&mut iter));
        }

        let mut result = Vec::new();

        Self::helper(&nums, &ops, &mut |value| result.push(value));

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
