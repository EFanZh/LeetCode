pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut nums = nums;
        let mut value = (1 << maximum_bit) - 1;

        for num in &mut nums {
            value ^= *num;
            *num = value;
        }

        nums.reverse();

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        Self::get_maximum_xor(nums, maximum_bit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
