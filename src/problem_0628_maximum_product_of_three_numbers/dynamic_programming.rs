pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let (left, right) = nums.split_at(2);
        let [first, second]: [i32; 2] = left.try_into().unwrap();

        let (mut min_1, mut max_1) = if first < second {
            (first, second)
        } else {
            (second, first)
        };

        let mut min_2 = first * second;
        let mut max_2 = min_2;
        let mut max_3 = i32::MIN;

        for &num in right {
            if num < 0 {
                max_3 = max_3.max(min_2 * num);
                min_2 = min_2.min(max_1 * num);
                max_2 = max_2.max(min_1 * num);
            } else {
                max_3 = max_3.max(max_2 * num);
                min_2 = min_2.min(min_1 * num);
                max_2 = max_2.max(max_1 * num);
            }

            max_1 = max_1.max(num);
            min_1 = min_1.min(num);
        }

        max_3
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_product(nums: Vec<i32>) -> i32 {
        Self::maximum_product(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
