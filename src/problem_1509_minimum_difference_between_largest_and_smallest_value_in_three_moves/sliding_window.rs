pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp;
use std::convert::TryInto;

impl Solution {
    fn insert(buffer: &mut [i32; 8], value: i32) {
        // The first 4 elements are the smallest 4 elements.
        // The last 4 elements are the largest 4 elements.

        if value < buffer[3] {
            if value < buffer[2] {
                buffer[3] = buffer[2];

                if value < buffer[1] {
                    buffer[2] = buffer[1];

                    if value < buffer[0] {
                        buffer[1] = buffer[0];
                        buffer[0] = value;
                    } else {
                        buffer[1] = value;
                    }
                } else {
                    buffer[2] = value;
                }
            } else {
                buffer[3] = value;
            }
        } else if value > buffer[4] {
            if value > buffer[5] {
                buffer[4] = buffer[5];

                if value > buffer[6] {
                    buffer[5] = buffer[6];

                    if value > buffer[7] {
                        buffer[6] = buffer[7];
                        buffer[7] = value;
                    } else {
                        buffer[6] = value;
                    }
                } else {
                    buffer[5] = value;
                }
            } else {
                buffer[4] = value;
            }
        }
    }

    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let nums = nums.as_mut_slice();
        let n = nums.len();

        if n < 5 {
            0
        } else {
            let (left, right) = nums.split_at_mut(n.min(8));

            left.sort_unstable();

            let selected = if left.len() == 8 {
                let elements: &mut [_; 8] = left.try_into().unwrap();

                for &num in &*right {
                    Self::insert(elements, num);
                }

                elements
            } else {
                left
            };

            let selected_length = selected.len();

            cmp::min(
                cmp::min(
                    selected[selected_length - 4] - selected[0],
                    selected[selected_length - 3] - selected[1],
                ),
                cmp::min(
                    selected[selected_length - 2] - selected[2],
                    selected[selected_length - 1] - selected[3],
                ),
            )
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_difference(nums: Vec<i32>) -> i32 {
        Self::min_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
