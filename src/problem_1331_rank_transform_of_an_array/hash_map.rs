pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut values = arr
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        values.sort_unstable();

        let ranks = values.into_iter().zip(1..).collect::<HashMap<_, _>>();

        let mut result = arr;

        for value in &mut result {
            *value = ranks[value];
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        Self::array_rank_transform(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
