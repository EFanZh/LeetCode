pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::ops::ControlFlow;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut indices = HashMap::with_capacity(nums.len());

        (0..)
            .zip(nums)
            .try_for_each(|(i, num)| {
                if let Some(old_index) = indices.insert(num, i)
                    && i - old_index <= k
                {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            })
            .is_break()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        Self::contains_nearby_duplicate(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
