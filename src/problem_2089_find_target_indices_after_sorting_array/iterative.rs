pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut less = 0;
        let mut equal = 0;

        for num in nums {
            match num.cmp(&target) {
                Ordering::Less => less += 1,
                Ordering::Equal => equal += 1,
                Ordering::Greater => {}
            }
        }

        (less..less + equal).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::target_indices(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
