pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut states = [1_i8; 100];
        let mut result = 0;

        for num in nums {
            let state = &mut states[num as u32 as usize - 1];

            result += num * i32::from(*state);
            *state = -i8::from(*state == 1);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_unique(nums: Vec<i32>) -> i32 {
        Self::sum_of_unique(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
