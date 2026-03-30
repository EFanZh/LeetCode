pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut max_negative = i32::MIN;
        let mut iter = nums.iter().copied();

        let mut result = loop {
            if let Some(num) = iter.next() {
                if num < 0 {
                    max_negative = max_negative.max(num);
                } else {
                    break num.cast_unsigned();
                }
            } else {
                return max_negative;
            }
        };

        let mut buckets = [0_usize; usize::div_ceil(100, usize::BITS as _)];

        buckets[(result / usize::BITS) as usize] |= 1 << (result % usize::BITS);

        for num in iter {
            if num > 0 {
                let num = num.cast_unsigned();
                let bucket = &mut buckets[(num / usize::BITS) as usize];
                let probe = 1 << (num % usize::BITS);

                if *bucket & probe == 0 {
                    *bucket |= probe;
                    result += num;
                }
            }
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum(nums: Vec<i32>) -> i32 {
        Self::max_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
