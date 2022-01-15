pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::mem;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut result = HashSet::<i32>::new();
        let mut prev = HashSet::new();
        let mut current = HashSet::new();

        for &num in &arr {
            // Calculate subarrays ends with `num`.

            for &x in &prev {
                current.insert(x | num);
            }

            current.insert(num);

            // Add current results to `result`.

            result.extend(&current);

            // Next iteration.

            mem::swap(&mut prev, &mut current);

            current.clear();
        }

        result.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        Self::subarray_bitwise_o_rs(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
