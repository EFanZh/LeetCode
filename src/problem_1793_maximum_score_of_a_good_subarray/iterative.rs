pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let k = k.cast_unsigned() as usize;
        let mut result = nums[k].cast_unsigned();
        let mut left = k;
        let mut left_height = result;
        let mut right = k;
        let mut right_height = result;

        loop {
            let width = (right - left) as u32;

            if left_height < right_height {
                result = result.max(right_height * width);
                right += 1;

                if let Some(&next_right_height) = nums.get(right) {
                    let next_right_height = next_right_height.cast_unsigned();

                    right_height = right_height.min(next_right_height);
                } else {
                    loop {
                        result = result.max(left_height * (right as u32 - left as u32));
                        left = left.wrapping_sub(1);

                        if let Some(&next_left_height) = nums.get(left) {
                            left_height = left_height.min(next_left_height.cast_unsigned());
                        } else {
                            break;
                        }
                    }

                    break;
                }
            } else {
                result = result.max(left_height * width);
                left = left.wrapping_sub(1);

                if let Some(&next_left_height) = nums.get(left) {
                    let next_left_height = next_left_height.cast_unsigned();

                    left_height = left_height.min(next_left_height);
                } else {
                    loop {
                        result = result.max(right_height * (right as u32).wrapping_sub(left as _));
                        right += 1;

                        if let Some(&next_right_height) = nums.get(right) {
                            right_height = right_height.min(next_right_height.cast_unsigned());
                        } else {
                            break;
                        }
                    }

                    break;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        Self::maximum_score(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
