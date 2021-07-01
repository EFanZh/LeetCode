pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(ops.len());

        for op in ops {
            match op.parse().map_err(|_| op.as_str()) {
                Ok(value) => stack.push(value),
                Err("C") => {
                    stack.pop();
                }
                Err("D") => stack.push(stack.last().unwrap() * 2),
                Err(_) => stack.push(stack[stack.len() - 2] + stack.last().unwrap()),
            }
        }

        stack.iter().sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn cal_points(ops: Vec<String>) -> i32 {
        Self::cal_points(ops)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
