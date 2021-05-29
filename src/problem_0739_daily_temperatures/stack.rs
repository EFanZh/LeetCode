pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; t.len()];
        let mut stack = Vec::new();

        for (i, (target, &value)) in result.iter_mut().zip(&t).enumerate().rev() {
            while let Some(&top) = stack.last() {
                if t[top] <= value {
                    stack.pop();
                } else {
                    *target = (top - i) as _;

                    break;
                }
            }

            stack.push(i);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        Self::daily_temperatures(t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
