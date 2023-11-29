pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut heights = heights;

        for target in heights.iter_mut().rev() {
            let mut count = 0;

            while let Some(&top) = stack.last() {
                count += 1;

                if top < *target {
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(*target);
            *target = count;
        }

        heights
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        Self::can_see_persons_count(heights)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
