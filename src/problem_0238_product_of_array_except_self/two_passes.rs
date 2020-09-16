pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut product = 1;

        for (target, num) in result.iter_mut().zip(&nums) {
            *target = product;
            product *= num;
        }

        product = 1;

        for (target, num) in result.iter_mut().zip(&nums).rev() {
            *target *= product;
            product *= num;
        }

        result
    }
}

impl super::Solution for Solution {
    fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        Self::product_except_self(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
