pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ptr;

impl Solution {
    fn address(value: &i64) -> usize {
        ptr::from_ref(value) as _
    }

    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        let mut result = vec![0; arr.len()];
        let mut indices = HashMap::<_, Vec<_>>::new();

        for (num, target) in arr.into_iter().zip(&mut result) {
            match indices.entry(num) {
                Entry::Occupied(entry) => entry.into_mut().push(target),
                Entry::Vacant(entry) => {
                    entry.insert(vec![target]);
                }
            }
        }

        for targets in indices.values_mut() {
            let mut sum = targets.iter().fold(0, |sum, index| sum + Self::address(index) as u64);
            let mut count_diff = (targets.len() as u64).wrapping_neg();
            let mut prev = 0;

            for target in targets {
                let target = &mut **target;
                let address = Self::address(target) as u64;

                sum = sum.wrapping_add((address - prev).wrapping_mul(count_diff));
                count_diff = count_diff.wrapping_add(2);
                prev = address;

                *target = (sum / (u64::from(usize::BITS / 8))) as _;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        Self::get_distances(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
