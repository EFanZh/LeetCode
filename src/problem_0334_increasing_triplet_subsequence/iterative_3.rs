pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for num in nums {
            if num > second {
                return true;
            } else if num > first {
                second = num;
            } else {
                first = num;
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn increasing_triplet(nums: Vec<i32>) -> bool {
        Self::increasing_triplet(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
