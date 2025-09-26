pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn reduce_xor(init: i32, nums: &[i32]) -> i32 {
        nums.iter().fold(init, |xor, value| xor ^ value)
    }

    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (init, nums) = match (nums1.len().is_multiple_of(2), nums2.len().is_multiple_of(2)) {
            (false, false) => (Self::reduce_xor(0, &nums1), &nums2),
            (false, true) => (0, &nums2),
            (true, false) => (0, &nums1),
            (true, true) => return 0,
        };

        Self::reduce_xor(init, nums)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::xor_all_nums(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
