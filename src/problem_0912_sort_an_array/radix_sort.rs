pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn min_max(nums: &[i32]) -> Option<(i32, i32)> {
        let mut iter = nums.iter().copied();

        iter.next().map(|first| {
            let mut min = first;
            let mut max = first;

            while let Some(left) = iter.next() {
                if let Some(right) = iter.next() {
                    let (new_min, new_max) = if right < left { (right, left) } else { (left, right) };

                    min = min.min(new_min);
                    max = max.max(new_max);
                } else {
                    if left < min {
                        min = left;
                    } else if left > max {
                        max = left;
                    }

                    break;
                }
            }

            (min, max)
        })
    }

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

    fn offset_by(nums: &mut [i32], offset: i32) {
        if offset != 0 {
            for num in nums {
                *num += offset;
            }
        }
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        if let Some((min, max)) = Self::min_max(&nums) {
            if min != max {
                Self::offset_by(&mut nums, -min);

                nums = Self::radix_sort(nums, max - min);

                Self::offset_by(&mut nums, min);
            }
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
