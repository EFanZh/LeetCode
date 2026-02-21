pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned();
        let mut masks = [0_usize; 101_usize.div_ceil(usize::BITS as usize)];

        for num in nums {
            match num.cmp(&k) {
                Ordering::Less => return -1,
                Ordering::Equal => {}
                Ordering::Greater => masks[(num / usize::BITS) as usize] |= 1 << (num % usize::BITS),
            }
        }

        masks.iter().copied().map(usize::count_ones).sum::<u32>().cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        Self::min_operations(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
