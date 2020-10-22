pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1 = nums1.into_iter().collect::<HashSet<_>>();
        let nums2 = nums2.into_iter().collect::<HashSet<_>>();

        nums1.into_iter().filter(|num| nums2.contains(num)).collect()
    }
}

impl super::Solution for Solution {
    fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        Self::intersection(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
