pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let n = nums.len() as u64;

        nums.sort_unstable();

        let mut sum = 0;

        let sums = nums
            .iter()
            .map(|&num| {
                sum += u64::from(num);

                sum
            })
            .collect::<Box<_>>();

        queries
            .into_iter()
            .map(|query| {
                let query = u64::from(query as u32);
                let index = nums.partition_point(|&num| num < query as u32);
                let left_sum = sums.get(index.wrapping_sub(1)).copied().unwrap_or(0);
                let index = index as u64;
                let left_operations = query * index - left_sum;
                let right_sum = sum - left_sum;
                let right_operations = right_sum - query * (n - index);

                (left_operations + right_operations) as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        Self::min_operations(nums, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
