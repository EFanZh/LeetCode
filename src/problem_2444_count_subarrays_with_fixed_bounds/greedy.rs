pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let min_k = min_k as u32;
        let max_k = max_k as u32;
        let mut start_index = 0;
        let mut min_index = 0;
        let mut max_index = 0;
        let mut result = 0;

        (1_u64..).zip(&nums).for_each(|(i, &num)| {
            if num < min_k || max_k < num {
                start_index = i;
            } else {
                if num == min_k {
                    min_index = i;
                }

                if num == max_k {
                    max_index = i;
                }

                result += min_index.min(max_index).saturating_sub(start_index);
            }
        });

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        Self::count_subarrays(nums, min_k, max_k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
