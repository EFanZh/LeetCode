pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_swaps(nums: Vec<i32>) -> i32 {
        let mut iter = nums.iter().copied();
        let mut result = 0;

        while iter.any(|num| num == 0) {
            if iter.rfind(|&num| num != 0).is_some() {
                result += 1;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_swaps(nums: Vec<i32>) -> i32 {
        Self::minimum_swaps(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
