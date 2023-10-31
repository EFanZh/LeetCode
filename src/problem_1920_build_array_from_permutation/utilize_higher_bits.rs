pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let slice = nums.as_mut_slice();

        for i in 0..slice.len() {
            slice[i] |= slice[slice[i] as u32 as usize] << 16;
        }

        for num in slice {
            *num >>= 16;
            *num &= 0x_ffff;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn build_array(nums: Vec<i32>) -> Vec<i32> {
        Self::build_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
