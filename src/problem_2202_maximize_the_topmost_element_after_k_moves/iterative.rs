pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32 as usize;

        if nums.len() < 2 {
            if k % 2 == 0 {
                nums[0]
            } else {
                -1
            }
        } else {
            let result = if let Some(&right) = nums.get(k) {
                if k < 2 {
                    return right;
                }

                right
            } else {
                i32::MIN
            };

            nums[..(k - 1).min(nums.len())]
                .iter()
                .fold(result, |max, &x| max.max(x))
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        Self::maximum_top(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
