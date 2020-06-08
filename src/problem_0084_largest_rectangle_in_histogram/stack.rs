pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut stack_base = Vec::<(i32, i32)>::new();
        let mut stack_top = (-1, 0);

        for item in (0..).zip(heights) {
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
}

impl super::Solution for Solution {
    fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        Self::largest_rectangle_area(heights)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
