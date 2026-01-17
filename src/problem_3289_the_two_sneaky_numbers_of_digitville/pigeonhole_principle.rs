pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len() - 2;
        let mut cursor = n;

        for i in 0..n {
            let mut current = nums[i];

            if current != i as i32 {
                loop {
                    let target = &mut nums[current.cast_unsigned() as usize];
                    let new_current = mem::replace(target, current);

                    if new_current == current {
                        current = mem::replace(&mut nums[cursor], new_current);
                        cursor += 1;
                    } else {
                        current = new_current;
                    }

                    if current == i as i32 {
                        nums[i] = current;

                        break;
                    }
                }
            }
        }

        nums.drain(..n);

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        Self::get_sneaky_numbers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
