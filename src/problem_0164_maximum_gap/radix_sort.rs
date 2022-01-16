pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn radix_sort(mut nums: Vec<i32>, max: i32) -> Vec<i32> {
        // From the book Introduction to Algorithms, third edition, page 199.

        let n = nums.len();
        let num_bits = 32 - max.leading_zeros();
        let log2_n = mem::size_of_val(&n) as u32 * 8 - 1 - n.leading_zeros();
        let mask_bits = num_bits.min(log2_n);
        let mask = (1 << mask_bits) - 1;
        let mut counts = vec![0_u32; 1 << mask_bits];
        let mut offset = 0;
        let mut temp = vec![0; n];

        loop {
            // Radix sort `n` `b`-bit numbers with  ⌈`b` / `r`⌉ `r`-bit digits.

            // Count numbers.

            for num in &*nums {
                counts[((num >> offset) & mask) as usize] += 1;
            }

            // Calculate indices.

            for i in 1..counts.len() {
                counts[i] += counts[i - 1];
            }

            // Place result into `temp`.

            for &num in nums.iter().rev() {
                let key = ((num >> offset) & mask) as usize;

                temp[counts[key] as usize - 1] = num;
                counts[key] -= 1;
            }

            // Reset counters.

            for count in &mut counts {
                *count = 0;
            }

            mem::swap(&mut nums, &mut temp);

            offset += mask_bits;

            if offset >= num_bits {
                break;
            }
        }

        nums
    }

    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let n = nums.len() as u32;
        let max = nums.iter().copied().max().unwrap();

        if max > 0 && n > 1 {
            let nums = Self::radix_sort(nums, max);

            nums.iter()
                .zip(&nums[1..])
                .map(|(previous, current)| current - previous)
                .max()
                .unwrap()
        } else {
            0
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_gap(nums: Vec<i32>) -> i32 {
        Self::maximum_gap(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
