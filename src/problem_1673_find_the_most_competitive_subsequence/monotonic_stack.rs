pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums;
        let k = k as u32 as usize;
        let buffer = nums.as_mut_slice();
        let n = buffer.len();
        let mut i = 0;
        let mut stack_length = 0;

        while let Some(&value) = buffer.get(i) {
            while i - stack_length < n - k {
                if let Some(&top) = buffer.get(stack_length.wrapping_sub(1)) {
                    if value < top {
                        stack_length -= 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if stack_length < k {
                buffer[stack_length] = value;
                stack_length += 1;
            }

            i += 1;
        }

        nums.truncate(k);

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::most_competitive(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
