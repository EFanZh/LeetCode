pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums2_count = HashMap::new();

        for num in nums2 {
            nums2_count.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut nums1 = nums1;

        nums1.retain(|num| {
            if let Entry::Occupied(entry) = nums2_count.entry(*num) {
                if *entry.get() == 1 {
                    entry.remove();
                } else {
                    *entry.into_mut() -= 1;
                }

                true
            } else {
                false
            }
        });

        nums1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        Self::intersect(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
