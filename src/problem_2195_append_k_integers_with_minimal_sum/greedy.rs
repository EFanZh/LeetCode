pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut k = u64::from(k as u32);

        nums.sort_unstable();

        let mut prev = 0;
        let mut candidate = k * (k + 1) / 2;

        for num in nums {
            if num != prev {
                if num <= k as u32 {
                    k += 1;
                    candidate += k - u64::from(num);
                } else {
                    break;
                }

                prev = num;
            }
        }

        candidate as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        Self::minimal_k_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
