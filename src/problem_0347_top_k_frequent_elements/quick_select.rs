pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as _;
        let mut counts = HashMap::with_hasher(BuildHasherDefault::<DefaultHasher>::default());

        for num in nums {
            counts.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut counts = counts.into_iter().collect::<Box<_>>();
        let mut start = 0;
        let mut end = counts.len();

        while end - start > 1 {
            let mut i = start;
            let mut j = end - 1;
            let pivot = counts[j].1;

            loop {
                if counts[i].1 > pivot {
                    i += 1;
                } else {
                    j -= 1;
                    counts.swap(i, j);
                }

                if i == j {
                    break;
                }
            }

            counts.swap(i, end - 1);

            match i.cmp(&k) {
                Ordering::Less => start = i + 1,
                Ordering::Equal => break,
                Ordering::Greater => end = i,
            }
        }

        counts[..k].iter().map(|&(num, _)| num).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::top_k_frequent(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
