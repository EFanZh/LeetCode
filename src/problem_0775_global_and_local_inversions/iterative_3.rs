pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        if let Some(right) = nums.get(2..) {
            let mut max = 0;

            for (&left, &right) in nums.iter().zip(right) {
                max = max.max(left);

                if right < max {
                    return false;
                }
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        Self::is_ideal_permutation(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
