pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::num::NonZeroU64;

impl Solution {
    fn count_triplets(counts: &mut HashMap<u64, u32>, result: &mut u32, left: &[i32], right: &[i32]) {
        for &third in right {
            let third = NonZeroU64::new(u64::from(third as u32)).unwrap();

            for &first in left {
                let first_squared = u64::from(first as u32).pow(2);

                if first_squared % third == 0 {
                    let second = first_squared / third;

                    if let Some(&count) = counts.get(&second) {
                        *result += count;
                    }
                }
            }

            counts.entry(third.get()).and_modify(|count| *count += 1).or_insert(1);
        }
    }

    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1 = nums1.as_slice();
        let nums2 = nums2.as_slice();
        let mut counts = HashMap::<u64, u32>::new();
        let mut result = 0;

        Self::count_triplets(&mut counts, &mut result, nums1, nums2);

        counts.clear();

        Self::count_triplets(&mut counts, &mut result, nums2, nums1);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::num_triplets(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
