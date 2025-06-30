pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let mut stack = Vec::<u64>::new();

        for &num in nums.iter().rev() {
            let mut num = u64::from(num as u32);

            while let Some(&top) = stack.last() {
                if top < num {
                    break;
                }

                num += top;
                stack.pop();
            }

            stack.push(num);
        }

        stack.iter().fold(0, |max, &num| max.max(num)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_array_value(nums: Vec<i32>) -> i64 {
        Self::max_array_value(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
