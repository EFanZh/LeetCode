pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut number_counts = HashMap::<_, u16>::new();

        for value in arr {
            number_counts.entry(value).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut occurrences = HashSet::new();

        number_counts.into_values().all(|count| occurrences.insert(count))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn unique_occurrences(arr: Vec<i32>) -> bool {
        Self::unique_occurrences(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
