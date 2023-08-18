pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut products = HashMap::<_, u32>::new();
        let mut iter = nums.iter().copied();
        let mut result = 0;

        while let Some(lhs) = iter.next() {
            for rhs in iter.clone() {
                products
                    .entry(lhs * rhs)
                    .and_modify(|count| {
                        result += *count * 8;

                        *count += 1;
                    })
                    .or_insert(1);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn tuple_same_product(nums: Vec<i32>) -> i32 {
        Self::tuple_same_product(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
