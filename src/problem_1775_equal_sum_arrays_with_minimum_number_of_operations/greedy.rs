pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn count_nums(nums: Vec<i32>) -> [u32; 6] {
        let mut result = [0; 6];

        for num in nums {
            result[num as u32 as usize - 1] += 1;
        }

        result
    }

    fn calculate_sum(counts: &[u32; 6]) -> u32 {
        let mut result = 0;
        let mut i = 0;

        for count in counts {
            i += 1;
            result += i * count;
        }

        result
    }

    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut counts_1 = Self::count_nums(nums1);
        let sums_1 = Self::calculate_sum(&counts_1);
        let mut counts_2 = Self::count_nums(nums2);
        let sums_2 = Self::calculate_sum(&counts_2);

        let mut diff = if sums_2 < sums_1 {
            mem::swap(&mut counts_1, &mut counts_2);

            sums_1 - sums_2
        } else {
            sums_2 - sums_1
        };

        let mut iter = counts_1.iter().zip(counts_2.iter().rev()).map(|(lhs, rhs)| lhs + rhs);
        let mut gain = 6;
        let mut result = 0;

        while diff != 0 {
            if let Some(count) = iter.next() {
                gain -= 1;

                if let Some(required) = (diff + gain - 1).checked_div(gain) {
                    if count < required {
                        result += count;

                        diff -= gain * count;
                    } else {
                        result += required;

                        break;
                    }
                }
            } else {
                return -1;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::min_operations(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
