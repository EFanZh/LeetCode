pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut result = 0_u32;

        let mut heights = (0..matrix.first().map_or(0, Vec::len) as u32)
            .map(|i| (0_u32, i))
            .collect::<Box<_>>();

        for row in matrix {
            // Sort heights in reverse order.

            let mut sorted = 0;
            let mut i = 0;

            while let Some(item) = heights.get_mut(i) {
                if row[item.1 as usize] == 0 {
                    item.0 = 0;
                } else {
                    item.0 += 1;

                    heights.swap(sorted, i);
                    sorted += 1;
                }

                i += 1;
            }

            // Calculate max area.

            for (length, &(height, _)) in (1..=sorted as u32).zip(&*heights) {
                result = result.max(length * height);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        Self::largest_submatrix(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
