pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let (nums, k, _) = (
            nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>(),
            k.cast_unsigned(),
            edges,
        );

        let mut sum = 0;
        let mut odd_count = false;
        let mut min_loss = u64::MAX;

        for num in nums {
            let num_xor = u64::from(num ^ k);
            let num = u64::from(num);

            sum += num.max(num_xor);

            if num_xor > num {
                odd_count = !odd_count;
            }

            min_loss = min_loss.min(num.abs_diff(num_xor));
        }

        (if odd_count { sum - min_loss } else { sum }).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        Self::maximum_value_sum(nums, k, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
