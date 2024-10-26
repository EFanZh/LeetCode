pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut operations = operations;
        let mut replacements = HashMap::new();

        while let Some(operation) = operations.pop() {
            let [from, to]: [_; 2] = operation.try_into().ok().unwrap();
            let to = replacements.get(&to).copied().unwrap_or(to);

            replacements.insert(from, to);
        }

        drop(operations);

        for num in &mut nums {
            if let Some(replacement) = replacements.get(num) {
                *num = *replacement;
            }
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        Self::array_change(nums, operations)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
