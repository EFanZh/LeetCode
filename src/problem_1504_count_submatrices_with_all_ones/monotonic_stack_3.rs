pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

struct Item {
    index: u8,
    height: u8,
    sum: u16,
}

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let columns = mat.first().map_or(0, Vec::len);
        let mut heights = vec![0_u8; columns].into_boxed_slice();
        let mut stack = Vec::<Item>::new();
        let mut result = 0_u32;

        for row in mat {
            for ((index, height_slot), value) in (0_u8..).zip(&mut *heights).zip(row) {
                let height = if value == 0 { 0 } else { *height_slot + 1 };

                *height_slot = height;

                loop {
                    let (left_sum, left_index) = if let Some(top) = stack.last_mut() {
                        match height.cmp(&top.height) {
                            Ordering::Less => {
                                stack.pop();

                                continue;
                            }
                            Ordering::Equal => {
                                top.sum += u16::from(height) * u16::from(index - top.index);
                                top.index = index;

                                result += u32::from(top.sum);

                                break;
                            }
                            Ordering::Greater => (top.sum, top.index),
                        }
                    } else {
                        (0, u8::MAX)
                    };

                    let sum = left_sum + u16::from(height) * u16::from(index.wrapping_sub(left_index));

                    stack.push(Item { index, height, sum });

                    result += u32::from(sum);

                    break;
                }
            }

            stack.clear();
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        Self::num_submat(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
