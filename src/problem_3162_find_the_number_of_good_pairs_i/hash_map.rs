pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let nums1 = nums1.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let nums2 = nums2.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned();
        let max = nums1.iter().copied().max().unwrap_or(0);
        let mut counts_1 = vec![0_u32; max as usize + 1].into_boxed_slice();

        for num in nums1 {
            counts_1[num as usize] += 1;
        }

        let mut counts_2 = HashMap::new();

        for num in nums2 {
            match counts_2.entry(num * k) {
                Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1_u32);
                }
            }
        }

        counts_2
            .into_iter()
            .filter_map(|(factor, count)| {
                let factor = factor as usize;

                counts_1
                    .get(factor..)
                    .map(|range| range.iter().step_by(factor).copied().map(u64::from).sum::<u64>() * u64::from(count))
            })
            .sum::<u64>()
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        Self::number_of_pairs(nums1, nums2, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
