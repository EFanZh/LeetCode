pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn i32_to_u32(value: i32) -> u32 {
        u32::from_le_bytes(value.wrapping_sub(i32::min_value()).to_le_bytes())
    }

    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as u32;
        let t = t as u32;
        let bucket_size = t + 1;
        let split = nums.len().min((k + 1) as _);
        let mut visited = HashMap::with_capacity(split);
        let (left, right) = nums.split_at(split);

        for num in left.iter().copied().map(Self::i32_to_u32) {
            let bucket = num / bucket_size;

            if visited.insert(num / bucket_size, num).is_some() {
                return true;
            }

            if let Some(j) = bucket.checked_sub(1) {
                if let Some(&value) = visited.get(&j) {
                    if num - value <= t {
                        return true;
                    }
                }
            }

            if let Some(j) = bucket.checked_add(1) {
                if let Some(&value) = visited.get(&j) {
                    if value - num <= t {
                        return true;
                    }
                }
            }
        }

        for (i, num) in right.iter().copied().map(Self::i32_to_u32).enumerate() {
            visited.remove(&(Self::i32_to_u32(nums[i]) / bucket_size));

            let bucket = num / bucket_size;

            if visited.insert(bucket, num).is_some() {
                return true;
            }

            if let Some(j) = bucket.checked_sub(1) {
                if let Some(&value) = visited.get(&j) {
                    if num - value <= t {
                        return true;
                    }
                }
            }

            if let Some(j) = bucket.checked_add(1) {
                if let Some(&value) = visited.get(&j) {
                    if value - num <= t {
                        return true;
                    }
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        Self::contains_nearby_almost_duplicate(nums, k, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
