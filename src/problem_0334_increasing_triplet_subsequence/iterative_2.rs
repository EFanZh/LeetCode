pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::max_value();
        let mut second = i32::max_value();

        for num in nums {
            if num <= first {
                first = num;
            } else if num <= second {
                second = num;
            } else {
                return true;
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
