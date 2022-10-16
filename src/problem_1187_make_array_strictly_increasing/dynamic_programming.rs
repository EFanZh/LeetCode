pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2 = arr2;

        arr2.sort_unstable();
        arr2.dedup();

        let mut cache = vec![i32::MAX; arr1.len() + 1];

        cache[0] = i32::MIN;

        for (i, value) in (2..).zip(arr1.into_iter()) {
            let mut prev = i32::MAX;

            for slot in &mut cache[..i] {
                let mut tail = if *slot < value { value } else { i32::MAX };

                tail = tail.min(
                    arr2.get(arr2.partition_point(|&replace_with| replace_with <= prev))
                        .copied()
                        .unwrap_or(i32::MAX),
                );

                prev = mem::replace(slot, tail);
            }
        }

        cache.iter().position(|&tail| tail != i32::MAX).unwrap_or(usize::MAX) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        Self::make_array_increasing(arr1, arr2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
