pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let (&first, rest) = nums.split_first().unwrap();
        let mut max = 0;
        let mut prev = first;

        for &num in rest {
            if num < max {
                return false;
            }

            max = max.max(prev);
            prev = num;
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
