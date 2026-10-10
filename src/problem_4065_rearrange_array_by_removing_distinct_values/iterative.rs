pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len() as u8;
        let mut counts = [0_u8; 100];

        for &num in &nums {
            counts[num.cast_unsigned() as usize - 1] += 1;
        }

        nums.clear();

        while (nums.len() as u8) < n {
            (1..).zip(&mut counts).for_each(|(i, count)| {
                if *count != 0 {
                    *count -= 1;
                    nums.push(i);
                }
            });
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        Self::rearrange_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
