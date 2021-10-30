pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        const MARKER: i32 = 1 << 31;

        let mut result = Vec::new();
        let mut nums = nums;

        for i in 0..nums.len() {
            let num = nums[i] & !MARKER;
            let target = &mut nums[(num - 1) as usize];

            if *target & MARKER == 0 {
                *target |= MARKER;
            } else {
                result.push(num);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        Self::find_duplicates(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
