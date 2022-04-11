pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();

        for (i, &num) in (0_u32..).zip(&nums) {
            if stack.last().map_or(true, |&(_, top)| num < top) {
                stack.push((i, num));
            }
        }

        let mut result = 0;
        let mut j = nums.len() as u32;
        let mut max = i32::MIN;

        for &num in nums.iter().rev() {
            j -= 1;

            if num > max {
                max = num;

                while let Some(&(i, top)) = stack.last() {
                    if top <= num {
                        stack.pop();
                        result = result.max(j - i);
                    } else {
                        break;
                    }
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_width_ramp(nums: Vec<i32>) -> i32 {
        Self::max_width_ramp(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
