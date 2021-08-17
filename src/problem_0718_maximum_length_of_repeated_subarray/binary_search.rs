pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

const D: u64 = 101;
const MODULUS: u64 = 91_320_515_216_383_913;

impl Solution {
    fn exp_mod(base: u64, exponent: usize, modulus: u64) -> u64 {
        // This algorithm has linear time complexity to avoid overflowing.

        let mut result = 1;

        for _ in 0..exponent {
            result = (result * base) % modulus;
        }

        result
    }

    fn rolling_hash<'a>(nums: &'a [i32], length: usize, h: u64, mut f: impl FnMut(u64, &'a [i32]) -> bool) -> bool {
        let (left, right) = nums.split_at(length);
        let mut hash = 0;

        for &num in left {
            hash = (hash * D + u64::from(num as u32)) % MODULUS;
        }

        if f(hash, left) {
            return true;
        }

        for (i, (&old, &new)) in (1..).zip(nums.iter().zip(right)) {
            let old = u64::from(old as u32);
            let new = u64::from(new as u32);

            hash = (D * (hash + (MODULUS - (h * old) % MODULUS)) + new) % MODULUS;

            if f(hash, &nums[i..i + length]) {
                return true;
            }
        }

        false
    }

    fn check<'a>(
        nums1: &'a [i32],
        nums2: &'a [i32],
        length: usize,
        buckets_buffer: &mut HashMap<u64, Vec<&'a [i32]>>,
        bucket_pool: &mut Vec<Vec<&'a [i32]>>,
    ) -> bool {
        let h = Self::exp_mod(D, length - 1, MODULUS);

        Self::rolling_hash(nums1, length, h, |hash, slice| {
            buckets_buffer
                .entry(hash)
                .and_modify(|v| v.push(slice))
                .or_insert_with(|| {
                    if let Some(mut v) = bucket_pool.pop() {
                        v.push(slice);

                        v
                    } else {
                        vec![slice]
                    }
                });

            false
        });

        let result = Self::rolling_hash(nums2, length, h, |hash, slice| {
            buckets_buffer.get(&hash).map_or(false, |v| v.contains(&slice))
        });

        for (_, mut bucket) in buckets_buffer.drain() {
            bucket.clear();
            bucket_pool.push(bucket);
        }

        result
    }

    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (nums1, nums2) = if nums2.len() < nums1.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let mut left = 1;
        let mut right = nums1.len() + 1;
        let mut buckets_buffer = HashMap::with_capacity(nums1.len());
        let mut bucket_pool = Vec::with_capacity(nums1.len());

        while left < right {
            let middle = (left + right) / 2;

            if Self::check(&nums1, &nums2, middle, &mut buckets_buffer, &mut bucket_pool) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        (left - 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::find_length(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
