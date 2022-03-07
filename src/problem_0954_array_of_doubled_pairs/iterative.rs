pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    fn partition(nums: &mut [i32], pivot: i32) -> (usize, usize) {
        // | less | garbage | unknown | greater |
        //         ^         ^         ^
        //         i         j         k

        let mut i = 0;
        let mut j = 0;
        let mut k = nums.len();

        while j < k {
            let num = nums[j];

            match num.cmp(&pivot) {
                Ordering::Less => {
                    nums[i] = num;
                    i += 1;
                    j += 1;
                }
                Ordering::Equal => j += 1,
                Ordering::Greater => {
                    k -= 1;
                    nums.swap(j, k);
                }
            }
        }

        nums[i..j].fill(pivot);

        (i, k)
    }

    fn remove(counts: &mut HashMap<i32, u32>, key: i32) -> bool {
        match counts.entry(key) {
            Entry::Occupied(entry) => {
                if *entry.get() == 0 {
                    entry.remove();
                } else {
                    *entry.into_mut() -= 1;
                }

                true
            }
            Entry::Vacant(_) => false,
        }
    }

    fn helper(nums: &mut [i32], counts: &mut HashMap<i32, u32>) -> bool {
        nums.sort_unstable();

        for &num in &*nums {
            counts.entry(num).and_modify(|count| *count += 1).or_insert(0);
        }

        for &num in &*nums {
            if Self::remove(counts, num) && !Self::remove(counts, num * 2) {
                return false;
            }
        }

        true
    }

    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        let (i, j) = Self::partition(&mut arr, 0);

        if i % 2 == 0 && (j - i) % 2 == 0 && (arr.len() - j) % 2 == 0 {
            let (left, rest) = arr.split_at_mut(i);
            let right = &mut rest[j - i..];

            for num in &mut *left {
                *num = -*num;
            }

            let mut counts = HashMap::<_, u32>::with_capacity(left.len().max(right.len()));

            if !Self::helper(left, &mut counts) {
                return false;
            }

            counts.clear();

            Self::helper(right, &mut counts)
        } else {
            false
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        Self::can_reorder_doubled(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
