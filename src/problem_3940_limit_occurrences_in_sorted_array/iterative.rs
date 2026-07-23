pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn limit_occurrences(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums;
        let mut prev = 0;
        let mut x = 0;

        nums.retain(|&num| {
            if num == prev {
                if x == 1 {
                    return false;
                }

                x -= 1;
            } else {
                prev = num;
                x = k;
            }

            true
        });

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn limit_occurrences(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::limit_occurrences(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
