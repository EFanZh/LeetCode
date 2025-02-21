pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = HashMap::new();

        for num in nums {
            match counts.entry(num) {
                Entry::Occupied(entry) => *entry.into_mut() = true,
                Entry::Vacant(entry) => {
                    entry.insert(false);
                }
            }
        }

        counts
            .iter()
            .filter_map(|(&k, &v)| {
                if v || counts.contains_key(&(k - 1)) || counts.contains_key(&(k + 1)) {
                    None
                } else {
                    Some(k)
                }
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        Self::find_lonely(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
