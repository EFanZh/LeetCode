pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let half = candy_type.len() / 2;
        let mut unique_types = HashSet::with_capacity(half);

        for t in candy_type {
            if unique_types.insert(t) && unique_types.len() == half {
                break;
            }
        }

        unique_types.len() as _
    }
}

impl super::Solution for Solution {
    fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        Self::distribute_candies(candy_type)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
