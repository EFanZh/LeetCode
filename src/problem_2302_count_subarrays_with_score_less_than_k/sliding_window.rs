pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let k = k.cast_unsigned();
        let mut start = 0;
        let mut sum = 0;

        (1..).zip(&nums).fold(0, |result, (end, &num)| {
            sum += u64::from(num.cast_unsigned());

            while sum * (end - start) >= k {
                sum -= u64::from(nums[start as usize].cast_unsigned());
                start += 1;
            }

            result + end - start
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        Self::count_subarrays(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
