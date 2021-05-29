pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn next_greater_elements(mut nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::with_capacity(nums.len());

        for &num in nums.iter().rev() {
            while stack.last().map_or(false, |&top| top <= num) {
                stack.pop();
            }

            stack.push(num);
        }

        for num in nums.iter_mut().rev() {
            let greater = loop {
                if let Some(&top) = stack.last() {
                    if top <= *num {
                        stack.pop();
                    } else {
                        break top;
                    }
                } else {
                    break -1;
                }
            };

            stack.push(mem::replace(num, greater));
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        Self::next_greater_elements(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
