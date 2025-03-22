pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_jump(stones: Vec<i32>) -> i32 {
        let mut iter = stones.iter().copied();
        let mut prev_forward = iter.next().unwrap();
        let mut prev_backward = prev_forward;
        let mut result = 0;

        while let Some(forward) = iter.next() {
            result = result.max(forward - prev_forward);
            prev_forward = forward;

            if let Some(backward) = iter.next() {
                result = result.max(backward - prev_backward);
                prev_backward = backward;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_jump(stones: Vec<i32>) -> i32 {
        Self::max_jump(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
