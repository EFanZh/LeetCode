pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn max_product_pair(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::new();
        let mut max_product = i32::MIN;
        let mut result = [-1, -1];

        (0..).zip(nums).for_each(|(i, num)| {
            let other = target - num;

            if other != num
                && let Some(&prev) = map.get(&other)
            {
                let product = num * other;

                if product > max_product {
                    max_product = product;

                    result = if num < other { [prev, i] } else { [i, prev] };
                }
            }

            map.insert(num, i);
        });

        Vec::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_product_pair(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::max_product_pair(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
