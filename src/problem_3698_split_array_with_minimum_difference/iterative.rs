pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut prev_1 = 0_u64;
        let mut candidate = 0_u64;
        let mut iter = nums.iter().copied();

        let mut prev_2 = u64::from(loop {
            if let Some(num) = iter.next() {
                if (prev_1 as u32) < num {
                    prev_1 = u64::from(num);
                    candidate += prev_1;
                } else {
                    break num;
                }
            } else {
                return (candidate - prev_1).abs_diff(prev_1).cast_signed();
            }
        });

        candidate = candidate.wrapping_sub(prev_2);

        for num in iter {
            if num < prev_2 as u32 {
                prev_2 = u64::from(num);
                candidate = candidate.wrapping_sub(prev_2);
            } else {
                return -1;
            }
        }

        let mut result = candidate.cast_signed().unsigned_abs();

        if prev_1 != prev_2 {
            result = result.min(candidate.wrapping_sub(prev_1 * 2).cast_signed().unsigned_abs());
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn split_array(nums: Vec<i32>) -> i64 {
        Self::split_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
