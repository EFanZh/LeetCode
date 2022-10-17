pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut min_sum = 0;
        let mut max_sum = 0;
        let mut max_subarray_sum = 0_u32;

        for value in arr {
            sum += value;
            min_sum = min_sum.min(sum);
            max_sum = max_sum.max(sum);
            max_subarray_sum = max_subarray_sum.max((sum - min_sum) as _);
        }

        let result = if k == 1 {
            max_subarray_sum
        } else {
            let base_sum = (max_sum - min_sum) as u32;

            if sum <= 0 {
                max_subarray_sum.max(base_sum.wrapping_add(sum as _))
            } else {
                ((u64::from(base_sum) + u64::from(sum as u32) * (u64::from(k as u32) - 1)) % 1_000_000_007) as _
            }
        };

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        Self::k_concatenation_max_sum(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
