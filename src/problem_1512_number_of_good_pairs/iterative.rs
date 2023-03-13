pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = [0; 100];
        let mut result = 0;

        for num in nums {
            let count = &mut count[num as u32 as usize - 1];

            result += *count;
            *count += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        Self::num_identical_pairs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
