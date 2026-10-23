pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::array;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MODULUS: u64 = 1_000_000_007;

        let k = u64::from(k.cast_unsigned());
        let mut bit_counts = array::from_fn::<_, 32, _>(|i| (0_u32, 1_u32 << i));

        for num in nums {
            let mut num = num.cast_unsigned();

            while num != 0 {
                bit_counts[num.trailing_zeros() as usize].0 += 1;
                num &= num - 1;
            }
        }

        bit_counts.sort_unstable_by_key(|&(count, _)| count);

        let split = bit_counts.partition_point(|&(count, _)| count == 0);
        let bit_counts = &mut bit_counts[split..];
        let mut num = 0;

        for (_, bit) in bit_counts.iter_mut().rev() {
            num |= *bit;
            *bit = num;
        }

        let mut prev_count = 0_u64;
        let mut result = 0;

        for &(count, num) in &*bit_counts {
            let count = u64::from(count);
            let num = u64::from(num);

            if prev_count != count {
                let num_squared = (num * num) % MODULUS;

                if (count as u32) < k as u32 {
                    result += num_squared * (count - prev_count);
                } else {
                    result += num_squared * (k - prev_count);

                    break;
                }

                prev_count = count;
            }
        }

        (result % MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        Self::max_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
