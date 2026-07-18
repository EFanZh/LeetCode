pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_opposite_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut total_odds = nums.iter().map(|&num| num & 1).sum::<i32>();
        let mut total_even = nums.len() as i32 - total_odds;

        for num in &mut nums {
            *num = if *num & 1 == 0 {
                total_even -= 1;
                total_odds
            } else {
                total_odds -= 1;
                total_even
            };
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_opposite_parity(nums: Vec<i32>) -> Vec<i32> {
        Self::count_opposite_parity(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
