pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

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

                let (left_index, left_sum) = loop {
                    if let Some(top) = stack.last() {
                        if height <= top.height {
                            stack.pop();
                        } else {
                            break (top.index, top.sum);
                        }
                    } else {
                        break (u8::MAX, 0);
                    }
                };

                let sum = left_sum + u16::from(height) * u16::from(index.wrapping_sub(left_index));

                result += u32::from(sum);

                stack.push(Item { index, height, sum });
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
