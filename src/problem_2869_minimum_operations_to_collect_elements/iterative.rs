pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let target = (1 << (k as u32 + 1)) - 2;
        let mut state = 0_u64;
        let mut result = 0;

        for &num in nums.iter().rev() {
            state |= 1 << num as u8;
            result += 1;

            if state & target == target {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        Self::min_operations(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
