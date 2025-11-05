pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        fn min(nums: &[i32]) -> i32 {
            nums.iter().copied().fold(i32::MAX, i32::min)
        }

        min(&nums2) - min(&nums1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::added_integer(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
