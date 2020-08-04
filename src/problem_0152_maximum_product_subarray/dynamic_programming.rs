pub struct Solution {}

use std::mem;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut iter = nums.into_iter();
        let mut result = iter.next().unwrap();
        let mut min_product = result;
        let mut max_product = result;

        for num in iter {
            if num < 0 {
                mem::swap(&mut min_product, &mut max_product);
            }

            min_product = (min_product * num).min(num);
            max_product = (max_product * num).max(num);
            result = result.max(max_product)
        }

        result
    }
}

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
