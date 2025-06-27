pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let mut max_offset = -1;
        let n = nums.len();

        for num in &mut nums {
            if *num == n as i32 - 1 {
                if max_offset == 1 {
                    return false;
                }

                *num += max_offset;
                max_offset += 1;
            } else {
                *num -= 1;
            }
        }

        if max_offset != 1 {
            return false;
        }

        for i in 0..nums.len() {
            let mut value = nums[i];

            if value != i as i32 {
                let start = i;

                loop {
                    let Some(next) = nums.get_mut(value as u32 as usize) else {
                        return false;
                    };

                    if *next == value {
                        return false;
                    }

                    value = mem::replace(next, value);

                    if value == start as i32 {
                        break;
                    }
                }
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_good(nums: Vec<i32>) -> bool {
        Self::is_good(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
