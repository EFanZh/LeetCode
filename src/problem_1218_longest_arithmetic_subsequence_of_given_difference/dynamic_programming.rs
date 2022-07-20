pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut result = 0;
        let mut cache = HashMap::new();

        for value in arr {
            let max_length = cache.get(&(value - difference)).map_or(1, |&length| length + 1);

            result = result.max(max_length);

            cache.insert(value, max_length);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        Self::longest_subsequence(arr, difference)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
