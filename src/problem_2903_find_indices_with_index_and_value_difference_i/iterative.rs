pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;
use std::ops::ControlFlow;

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let index_difference = index_difference as u32 as usize;

        if index_difference < nums.len() {
            let mut min = i32::MAX;
            let mut min_index = 0;
            let mut max = i32::MIN;
            let mut max_index = 0;

            iter::zip(
                iter::zip(0.., &nums),
                iter::zip(index_difference.., &nums[index_difference..]),
            )
            .try_for_each(|((i, &left), (j, &right))| {
                if left < min {
                    min = left;
                    min_index = i;
                }

                if left > max {
                    max = left;
                    max_index = i;
                }

                let diff_from_min = right - min;
                let diff_to_max = max - right;

                let (max_diff, max_diff_index) = if diff_to_max < diff_from_min {
                    (diff_from_min, min_index)
                } else {
                    (diff_to_max, max_index)
                };

                if max_diff < value_difference {
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Break([max_diff_index, j as _])
                }
            })
            .break_value()
        } else {
            None
        }
        .unwrap_or([-1, -1])
        .to_vec()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        Self::find_indices(nums, index_difference, value_difference)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
