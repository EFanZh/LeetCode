pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut diff = nums.iter().sum::<i32>();
        let mut result = 0;

        for num in nums {
            if num == 0 {
                result += match diff {
                    -1 | 1 => 1,
                    0 => 2,
                    ..-1 => break,
                    _ => continue,
                }
            } else {
                diff -= num * 2;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_valid_selections(nums: Vec<i32>) -> i32 {
        Self::count_valid_selections(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
