pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn abs_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut k = k.cast_unsigned() as usize;
        let n = nums.len();
        let half = n / 2;

        if k > half {
            k = n - k;
        }

        if k > 0 && k < n {
            nums.select_nth_unstable(k);

            if k < half {
                nums[k + 1..].select_nth_unstable(n - k * 2 - 1);
            }
        }

        (nums[n - k..].iter().sum::<u32>() - nums[..k].iter().sum::<u32>()).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn abs_difference(nums: Vec<i32>, k: i32) -> i32 {
        Self::abs_difference(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
