pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut stack = Vec::<(u32, u64)>::with_capacity(nums.len());

        let left_sums = nums
            .iter()
            .map(|&num| {
                let mut left_sum = 0;

                while let Some(&(top_num, top_sum)) = stack.last() {
                    if top_num >= num {
                        stack.pop();
                        left_sum += top_sum;
                    } else {
                        break;
                    }
                }

                stack.push((num, left_sum + u64::from(num)));

                left_sum
            })
            .collect::<Box<_>>();

        stack.clear();

        let mut result = 0;

        for (&num, &left_sum) in nums.iter().zip(&*left_sums).rev() {
            let num = u64::from(num);
            let mut right_sum = num;

            while let Some(&(top_num, top_sum)) = stack.last() {
                if top_num >= num as u32 {
                    stack.pop();
                    right_sum += top_sum;
                } else {
                    break;
                }
            }

            stack.push((num as u32, right_sum));

            result = result.max(num * (left_sum + right_sum));
        }

        (result % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        Self::max_sum_min_product(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
