pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = BinaryHeap::from(satisfaction);
        let mut sum = 0;
        let mut result = 0;

        while let Some(value) = satisfaction.pop() {
            sum += value;

            if sum <= 0 {
                break;
            }

            result += sum;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        Self::max_satisfaction(satisfaction)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
