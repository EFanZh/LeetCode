pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::BuildHasherDefault;

impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        if nums.len().is_multiple_of(k as usize) {
            let mut counts = HashMap::with_hasher(BuildHasherDefault::<DefaultHasher>::default());

            for num in nums {
                counts.entry(num).and_modify(|count| *count += 1).or_insert(1);
            }

            let mut chains = HashMap::new();
            let mut opened = 0;

            while let Some(mut num) = counts.keys().next().copied() {
                while counts.contains_key(&(num - 1)) {
                    num -= 1;
                }

                while let Some(count) = counts.remove(&num) {
                    if count < opened {
                        return false;
                    }

                    let extra = count - opened;

                    if extra != 0 {
                        chains.insert(num, extra);
                    }

                    opened = chains.remove(&(num - k + 1)).map_or(count, |closed| count - closed);

                    num += 1;
                }

                if opened != 0 {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        Self::is_possible_divide(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
