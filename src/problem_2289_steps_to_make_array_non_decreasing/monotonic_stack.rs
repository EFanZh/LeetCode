pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut top = (0_u32, 0_u32);
        let mut result = 0;

        for num in nums {
            let num = num as u32;
            let mut max_steps = 0;

            loop {
                if num < top.0 {
                    stack.push(top);
                    top = (num, max_steps + 1);
                    result = result.max(top.1);

                    break;
                }

                max_steps = max_steps.max(top.1);

                if let Some(new_top) = stack.pop() {
                    top = new_top;
                } else {
                    top.0 = num;

                    break;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn total_steps(nums: Vec<i32>) -> i32 {
        Self::total_steps(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
