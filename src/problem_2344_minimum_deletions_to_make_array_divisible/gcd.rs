pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn gcd(mut x: u32, mut y: u32) -> u32 {
        while y != 0 {
            (x, y) = (y, x % y);
        }

        x
    }

    fn partition_by(nums: &mut [u32], mut f: impl FnMut(u32) -> bool) -> usize {
        let mut result = 0;
        let mut iter = nums.iter_mut();

        'outer: while let Some(left) = iter.next() {
            if !f(*left) {
                loop {
                    if let Some(right) = iter.next_back() {
                        if f(*right) {
                            mem::swap(left, right);

                            break;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }

            result += 1;
        }

        result
    }

    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let vec_i32_to_u32 = |v: Vec<i32>| v.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut nums = vec_i32_to_u32(nums);
        let nums_divide = vec_i32_to_u32(nums_divide);
        let mut gcd = 0;

        for num in nums_divide {
            gcd = Self::gcd(num, gcd);
        }

        let nums = nums.as_mut_slice();
        let split = Self::partition_by(nums, |num| gcd % num == 0);
        let (left, right) = nums.split_at(split);

        left.iter().copied().min().map_or(-1, |min| {
            right.iter().fold(0, |count, &num| count + i32::from(num < min))
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        Self::min_operations(nums, nums_divide)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
