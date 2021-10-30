pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len() as i32;
        let mut extra = -1;

        for i in 0..n {
            loop {
                let num = &mut nums[i as usize];

                if *num != i && *num != -1 {
                    if *num == n {
                        mem::swap(num, &mut extra);
                    } else {
                        let j = *num;

                        nums.swap(i as _, j as _);
                    }
                } else {
                    break;
                }
            }
        }

        let mut i = 0;

        for num in nums {
            if num == -1 {
                break;
            }

            i += 1;
        }

        i
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn missing_number(nums: Vec<i32>) -> i32 {
        Self::missing_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
