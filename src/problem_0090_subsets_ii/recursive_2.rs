pub struct Solution {}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    fn subsets_with_dup_helper<'a>(
        mut counts: impl Iterator<Item = (&'a i32, &'a i32)> + Clone,
        base: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if let Some((&num, count)) = counts.next() {
            let saved_size = base.len();
            let mut count = *count;

            Self::subsets_with_dup_helper(counts.clone(), base, result);

            loop {
                base.push(num);

                Self::subsets_with_dup_helper(counts.clone(), base, result);

                if count == 1 {
                    break;
                } else {
                    count -= 1;
                }
            }

            base.truncate(saved_size);
        } else {
            result.push(base.clone());
        }
    }

    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut counts = HashMap::new();

        for num in nums {
            match counts.entry(num) {
                Entry::Occupied(entry) => *entry.into_mut() += 1,
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }

        let mut result = Vec::new();

        Self::subsets_with_dup_helper(counts.iter(), &mut Vec::new(), &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::subsets_with_dup(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
