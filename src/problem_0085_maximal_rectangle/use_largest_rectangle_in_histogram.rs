pub struct Solution;

impl Solution {
    fn largest_rectangle_area(stack_base: &mut Vec<(i32, i32)>, heights: &[i32]) -> i32 {
        let mut result = 0;
        let mut stack_top = (-1, 0);

        for item in (0..).zip(heights.iter().copied()) {
            loop {
                if item.1 <= stack_top.1 {
                    if let Some(new_top) = stack_base.pop() {
                        result = result.max((item.0 - new_top.0 - 1) * stack_top.1);
                        stack_top = new_top;
                    } else {
                        result = result.max(item.0 * stack_top.1);
                        stack_top = item;

                        break;
                    }
                } else {
                    stack_base.push(stack_top);
                    stack_top = item;

                    break;
                }
            }
        }

        let right = stack_top.0;

        while let Some(new_top) = stack_base.pop() {
            result = result.max((right - new_top.0) * stack_top.1);
            stack_top = new_top;
        }

        result
    }

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let mut heights = vec![0; matrix.first().map_or(0, Vec::len)];
        let mut stack_base = Vec::new();

        for row in matrix {
            for (height, row) in heights.iter_mut().zip(row) {
                if row == '0' {
                    *height = 0;
                } else {
                    *height += 1;
                }
            }

            result = result.max(Self::largest_rectangle_area(&mut stack_base, &heights));
        }

        result
    }
}

impl super::Solution for Solution {
    fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        Self::maximal_rectangle(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
