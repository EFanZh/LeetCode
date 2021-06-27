pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut s3 = i32::MIN;
        let mut stack = Vec::with_capacity(nums.len());

        for &num in nums.iter().rev() {
            if num < s3 {
                return true;
            }

            while let Some(&top) = stack.last().filter(|&&top| num > top) {
                s3 = top;
                stack.pop();
            }

            stack.push(num);
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find132pattern(nums: Vec<i32>) -> bool {
        Self::find132pattern(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
