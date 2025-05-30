pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn matrix_sum(nums: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;

        for num in &mut nums {
            num.sort_unstable_by(|&lhs, &rhs| u32::cmp(&(rhs as _), &(lhs as _)));
        }

        (0..nums.first().map_or(0, Vec::len))
            .map(|column| {
                nums.iter()
                    .map_while(|row| row.get(column).copied())
                    .fold(0, |max, value| u32::max(max, value as _))
            })
            .sum::<u32>() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn matrix_sum(nums: Vec<Vec<i32>>) -> i32 {
        Self::matrix_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
