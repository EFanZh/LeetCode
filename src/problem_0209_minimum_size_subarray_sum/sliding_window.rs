pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = 0;
        let mut sum = 0;
        let mut result = usize::max_value();

        while let Some(num) = nums.get(end) {
            end += 1;
            sum += num;

            if sum >= s {
                loop {
                    sum -= nums[start];

                    if sum < s {
                        result = result.min(end - start);
                        start += 1;

                        break;
                    }

                    start += 1;
                }
            }
        }

        if result == usize::max_value() {
            0
        } else {
            result as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        Self::min_sub_array_len(s, nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
