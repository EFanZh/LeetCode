pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut temp = 0;
        let mut i = 1;

        for num in nums {
            temp ^= i;
            temp ^= num;

            i += 1
        }

        temp
    }
}

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
