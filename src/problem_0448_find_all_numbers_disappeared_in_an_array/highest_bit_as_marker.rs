pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        const MARKER: i32 = 1 << 31;

        for i in 0..nums.len() {
            let index = ((nums[i] & !MARKER) - 1) as usize;

            nums[index] |= MARKER;
        }

        (1..=nums.len() as _)
            .filter(|i| nums[(i - 1) as usize] & MARKER == 0)
            .collect()
    }
}

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
