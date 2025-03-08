pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        let nums = nums.as_mut_slice();
        let n = nums.len();
        let mut usages = vec![0_i32; n].into_boxed_slice();

        for request in requests {
            let [start, end] = request.try_into().ok().unwrap();

            usages[start as u32 as usize] += 1;

            if let Some(diff) = usages.get_mut(end as u32 as usize + 1) {
                *diff -= 1;
            }
        }

        let mut usage = 0;

        for diff_or_usage in &mut *usages {
            usage += *diff_or_usage;
            *diff_or_usage = usage;
        }

        nums.sort_unstable();
        usages.sort_unstable();

        let result = nums
            .iter()
            .zip(&*usages)
            .map(|(&num, &usage)| i64::from(num) * i64::from(usage))
            .sum::<i64>();

        (result as u64 % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        Self::max_sum_range_query(nums, requests)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
