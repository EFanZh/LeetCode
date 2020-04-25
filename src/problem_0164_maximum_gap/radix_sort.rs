pub struct Solution {}

use std::mem;

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;

        if let Some(max_value) = nums.iter().max() {
            let n = nums.len();

            if *max_value > 0 && n > 1 {
                // From the book Introduction to Algorithms, third edition, page 199.

                let b = mem::size_of_val(max_value) * 8 - (max_value.leading_zeros() as usize);
                let log2_n = mem::size_of_val(&n) * 8 - (n.leading_zeros() as usize) - 1;
                let r = b.min(log2_n);
                let num_digits = (b + (r - 1)) / r;

                // Radix sort n b-bit numbers with  ⌈b / r⌉ r-bit digits.

                let mut temp_nums = vec![0; n];
                let mut counts = vec![0; 1 << r];
                let mask = (1 << r) - 1;

                for key_bit in (0..r * num_digits).step_by(r) {
                    // Sort by this digit with counting sort.

                    // Count numbers.

                    for value in &nums {
                        counts[((value >> key_bit) & mask) as usize] += 1;
                    }

                    // Calculate indices.

                    for i in 1..counts.len() {
                        counts[i] += counts[i - 1];
                    }

                    // Place result to temp_nums.

                    for value in nums.iter().rev() {
                        let key = ((value >> key_bit) & mask) as usize;

                        temp_nums[counts[key] - 1] = *value;

                        counts[key] -= 1;
                    }

                    // Maintain loop invariant.

                    for c in &mut counts {
                        *c = 0;
                    }

                    mem::swap(&mut nums, &mut temp_nums);
                }

                let mut iter = nums.into_iter();

                if let Some(mut previous) = iter.next() {
                    for current in iter {
                        result = result.max(current - previous);

                        previous = current
                    }
                }
            }
        }

        result
    }
}

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
