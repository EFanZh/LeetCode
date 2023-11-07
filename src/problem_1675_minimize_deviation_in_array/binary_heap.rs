pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut min = i32::MAX;

        for num in &mut nums {
            *num <<= *num & 1;
            min = min.min(*num);
        }

        let mut queue = BinaryHeap::from(nums);
        let mut result = i32::MAX;

        while let Some(mut top) = queue.peek_mut() {
            let top_value = *top;

            result = result.min(top_value - min);

            if top_value & 1 == 0 {
                let top_ref = &mut *top;

                *top_ref /= 2;

                min = min.min(*top_ref);
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_deviation(nums: Vec<i32>) -> i32 {
        Self::minimum_deviation(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
