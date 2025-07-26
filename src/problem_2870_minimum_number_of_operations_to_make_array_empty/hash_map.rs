pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::<_, u32>::new();

        for num in nums {
            counts.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut result = 0;

        for &value in counts.values() {
            if value == 1 {
                return -1;
            }

            result += value.div_ceil(3);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>) -> i32 {
        Self::min_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
