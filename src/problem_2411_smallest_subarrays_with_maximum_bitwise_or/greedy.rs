pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let mut one_indices = [0_u32; 32];
        let mut nums = nums;

        for (i, target) in nums.iter_mut().enumerate().rev() {
            let i = i as u32;
            let mut value = *target;

            while value != 0 {
                one_indices[value.trailing_zeros() as usize] = i;
                value &= value - 1;
            }

            *target = (one_indices.iter().copied().max().unwrap().saturating_sub(i) + 1) as _;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        Self::smallest_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
