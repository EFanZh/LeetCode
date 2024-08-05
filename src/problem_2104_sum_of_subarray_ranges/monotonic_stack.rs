pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut min_stack = Vec::<(i32, u32)>::new();
        let mut prev_min_length = 1;
        let mut max_stack = Vec::<(i32, u32)>::new();
        let mut iter = nums.into_iter();
        let mut prev_max_length = 1;
        let mut prev = iter.next().unwrap();
        let mut sum = 0;
        let mut result = 0;

        for num in iter {
            match num.cmp(&prev) {
                Ordering::Less => {
                    max_stack.push((prev, prev_max_length));
                    prev_max_length = 1;

                    sum += u64::from((prev - num) as u32) * u64::from(prev_min_length);
                    prev_min_length += 1;

                    while let Some(&(top, top_length)) = min_stack.last() {
                        if num <= top {
                            sum += u64::from((top - num) as u32) * u64::from(top_length);
                            prev_min_length += top_length;
                            min_stack.pop();
                        } else {
                            break;
                        }
                    }
                }
                Ordering::Equal => {
                    prev_min_length += 1;
                    prev_max_length += 1;
                }
                Ordering::Greater => {
                    min_stack.push((prev, prev_min_length));
                    prev_min_length = 1;

                    sum += u64::from((num - prev) as u32) * u64::from(prev_max_length);
                    prev_max_length += 1;

                    while let Some(&(top, top_length)) = max_stack.last() {
                        if num >= top {
                            sum += u64::from((num - top) as u32) * u64::from(top_length);
                            prev_max_length += top_length;
                            max_stack.pop();
                        } else {
                            break;
                        }
                    }
                }
            }

            prev = num;
            result += sum;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        Self::sub_array_ranges(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
