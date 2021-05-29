pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_indices = nums1
            .iter()
            .copied()
            .enumerate()
            .map(|(i, num)| (num, i))
            .collect::<HashMap<_, _>>();

        let mut stack = Vec::with_capacity(nums2.len());

        for &num in nums2.iter().rev() {
            let next_greater = loop {
                if let Some(&top) = stack.last() {
                    if top > num {
                        stack.push(num);

                        break top;
                    }

                    stack.pop();
                } else {
                    stack.push(num);

                    break -1;
                }
            };

            if let Some(&index) = nums1_indices.get(&num) {
                nums1[index] = next_greater;
            }
        }

        nums1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        Self::next_greater_element(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
