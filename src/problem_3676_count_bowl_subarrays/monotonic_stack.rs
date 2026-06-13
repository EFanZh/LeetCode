pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn bowl_subarrays(nums: Vec<i32>) -> i64 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let nums = nums.as_mut_slice();
        let mut result = 0;
        let mut stack_size = 0_usize;
        let mut i = 0;

        'outer: while let Some(&num) = nums.get(i) {
            i += 1;

            let mut top_index = stack_size.wrapping_sub(1);

            if let Some(top) = nums.get(top_index) {
                match u32::cmp(top, &num) {
                    Ordering::Less => loop {
                        stack_size = top_index;
                        top_index = top_index.wrapping_sub(1);

                        if let Some(top) = nums.get(top_index) {
                            result += 1;

                            match u32::cmp(top, &num) {
                                Ordering::Less => {}
                                Ordering::Equal => continue 'outer,
                                Ordering::Greater => break,
                            }
                        } else {
                            break;
                        }
                    },
                    Ordering::Equal => continue,
                    Ordering::Greater => {}
                }
            }

            nums[stack_size] = num;
            stack_size += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn bowl_subarrays(nums: Vec<i32>) -> i64 {
        Self::bowl_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
