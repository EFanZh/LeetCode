pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut min_1 = u32::MAX;
        let mut seen = 0_u16;

        for num in nums1 {
            let num = num.cast_unsigned();

            min_1 = min_1.min(num);
            seen |= 1 << num;
        }

        let mut min_same = u32::MAX;
        let mut min_2 = u32::MAX;

        for num in nums2 {
            let num = num.cast_unsigned();

            min_2 = min_2.min(num);

            if seen & (1 << num) != 0 {
                min_same = min_same.min(num);
            }
        }

        (if min_same == u32::MAX {
            if min_2 < min_1 {
                mem::swap(&mut min_1, &mut min_2);
            }

            10 * min_1 + min_2
        } else {
            min_same
        })
        .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::min_number(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
