pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned();
        let mut bit_counts = [0_u32; 32];

        for &num in &nums {
            let mut num = num;

            while num != 0 {
                bit_counts[num.trailing_zeros() as usize] += 1;
                num &= num - 1;
            }
        }

        let max_bits = 32 - bit_counts.iter().rev().take_while(|&&x| x == 0).count();
        let bit_counts = &bit_counts[..max_bits];

        nums.iter()
            .fold(0_u64, |max, &num| {
                let mut rest = 0;

                for (i, &bit_count) in bit_counts.iter().enumerate() {
                    let probe = 1 << i;

                    if bit_count - u32::from((num & probe) != 0) != 0 {
                        rest |= probe;
                    }
                }

                max.max((u64::from(num) << k) | u64::from(rest))
            })
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        Self::maximum_or(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
