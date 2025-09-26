pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len() as u32;

        (1..)
            .zip(nums)
            .filter(|&(i, _)| n.is_multiple_of(i))
            .map(|(_, num)| num * num)
            .sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_squares(nums: Vec<i32>) -> i32 {
        Self::sum_of_squares(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
