pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// <https://leetcode.com/problems/sum-of-subsequence-widths/discuss/161267/JavaC%2B%2BPython-Sort-and-One-Pass>.

impl Solution {
    pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
        const MODULUS: u32 = 1_000_000_007;

        let mut nums = nums;

        nums.sort_unstable();

        let mut result = 0;
        let mut count = 1;

        for (&high, &low) in nums.iter().zip(nums.iter().rev()) {
            let low = low as u32;
            let high = high as u32;
            let diff = high.checked_sub(low).unwrap_or(high + MODULUS - low);

            result += (u64::from(diff) * u64::from(count) % u64::from(MODULUS)) as u32;

            if result >= MODULUS {
                result -= MODULUS;
            }

            count *= 2;

            if count >= MODULUS {
                count -= MODULUS;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
        Self::sum_subseq_widths(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
