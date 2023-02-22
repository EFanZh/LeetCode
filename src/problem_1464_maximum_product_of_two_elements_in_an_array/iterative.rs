pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_1 = i32::MIN;
        let mut max_2 = i32::MIN;

        for num in nums {
            if num > max_1 {
                max_2 = max_1;
                max_1 = num;
            } else if num > max_2 {
                max_2 = num;
            }
        }

        (max_1 - 1) * (max_2 - 1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_product(nums: Vec<i32>) -> i32 {
        Self::max_product(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
