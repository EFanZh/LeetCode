pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let (mut nums, _) = (nums, n);
        let n = nums.len() / 2;

        for i in 0..n {
            nums[i * 2] |= nums[i] << 16;
            nums[i * 2 + 1] |= nums[n + i] << 16;
        }

        for target in &mut nums {
            *target >>= 16;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        Self::shuffle(nums, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
