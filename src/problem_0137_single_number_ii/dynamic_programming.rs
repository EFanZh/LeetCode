pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bits_1 = 0;
        let mut bits_2 = 0;

        for num in nums {
            // 00 => 01 => 01.
            // 01 => 00 => 10.
            // 10 => 10 => 00.

            bits_1 = (bits_1 ^ num) & !bits_2;
            bits_2 = (bits_2 ^ num) & !bits_1;
        }

        bits_1
    }
}

impl super::Solution for Solution {
    fn single_number(nums: Vec<i32>) -> i32 {
        Self::single_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
