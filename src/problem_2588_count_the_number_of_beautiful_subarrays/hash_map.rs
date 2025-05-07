pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::{BuildHasherDefault, Hasher};

#[derive(Default)]
struct FxHasher(usize);

impl Hasher for FxHasher {
    fn finish(&self) -> u64 {
        self.0.rotate_left(26) as _
    }

    fn write(&mut self, bytes: &[u8]) {
        for value in bytes.chunks_exact(4) {
            self.0 = self
                .0
                .wrapping_add(u32::from_ne_bytes(value.try_into().unwrap()) as usize)
                .wrapping_mul(0x_0000_f135_7aea_2e62_a9c5);
        }
    }
}

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut counts = HashMap::<i32, u32, BuildHasherDefault<FxHasher>>::with_capacity_and_hasher(
            nums.len(),
            BuildHasherDefault::new(),
        );

        counts.insert(0, 0);

        let mut state = 0;
        let mut result = 0;

        for num in nums {
            state ^= num;

            match counts.entry(state) {
                Entry::Occupied(occupied_entry) => {
                    let count = occupied_entry.into_mut();

                    *count += 1;

                    result += u64::from(*count);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(0);
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        Self::beautiful_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
