pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let ranks = (0_u32..)
            .zip(target)
            .map(|(i, num)| (num, i))
            .collect::<HashMap<_, _>>();

        let mut length_to_min_rank = Vec::with_capacity(ranks.len());

        for target in arr {
            if let Some(&rank) = ranks.get(&target) {
                let length = length_to_min_rank.partition_point(|&left_index| left_index < rank);

                #[allow(clippy::option_if_let_else)] // False positive.
                if let Some(min_rank) = length_to_min_rank.get_mut(length) {
                    *min_rank = rank;
                } else {
                    length_to_min_rank.push(rank);
                }
            }
        }

        (ranks.len() - length_to_min_rank.len()) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        Self::min_operations(target, arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
