pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        for i in 0..nums.len() {
            let mut num = nums[i];
            let expected = i as i32 + 1;

            while num != expected {
                let j = (num - 1) as _;
                let target = nums[j];

                if num == target {
                    break;
                }

                nums.swap(i, j);
                num = target;
            }
        }

        (1..=nums.len() as _).filter(|&i| nums[(i - 1) as usize] != i).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        Self::find_disappeared_numbers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
