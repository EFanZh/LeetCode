pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut buckets = vec![i64::MAX / 2; k.cast_unsigned() as _].into_boxed_slice();
        let mut bucket_index = 0;
        let mut result = i64::MIN;
        let mut sum = 0;

        buckets[0] = 0;

        for num in nums {
            bucket_index += 1;

            if bucket_index >= buckets.len() {
                bucket_index = 0;
            }

            let bucket = &mut buckets[bucket_index];

            sum += i64::from(num);
            result = result.max(sum - *bucket);
            *bucket = (*bucket).min(sum);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        Self::max_subarray_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
