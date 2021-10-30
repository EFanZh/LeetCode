pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut extra_xor_missing = 0;
        let mut nums = nums;
        let mut i = 0;

        let extra = loop {
            let num = nums[i] & i32::MAX;
            let slot = &mut nums[(num - 1) as usize];

            extra_xor_missing ^= num ^ ((i + 1) as i32);
            i += 1;

            if *slot & i32::MIN == 0 {
                *slot |= i32::MIN;
            } else {
                break num;
            }
        };

        while let Some(num) = nums.get(i) {
            extra_xor_missing ^= (num & i32::MAX) ^ ((i + 1) as i32);
            i += 1;
        }

        vec![extra, extra_xor_missing ^ extra]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        Self::find_error_nums(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
