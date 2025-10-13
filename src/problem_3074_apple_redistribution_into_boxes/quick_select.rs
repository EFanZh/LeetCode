pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn partition(values: &mut [u32], left: usize, right: usize, key: u32) -> usize {
        // +------+-------+-----------+---------+
        // | Less | Equal | Unchecked | Greater |
        // +------+-------+-----------+---------+

        let mut equal_start = left;
        let mut unchecked_start = left;
        let mut greater_start = right;

        while unchecked_start < greater_start {
            match key.cmp(&values[unchecked_start]) {
                Ordering::Less => {
                    values.swap(equal_start, unchecked_start);

                    equal_start += 1;
                    unchecked_start += 1;
                }
                Ordering::Equal => unchecked_start += 1,
                Ordering::Greater => {
                    greater_start -= 1;

                    values.swap(unchecked_start, greater_start);
                }
            }
        }

        left.midpoint(right - 1).clamp(equal_start, unchecked_start - 1)
    }

    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let total_apples = apple.into_iter().sum::<i32>().cast_unsigned();
        let mut capacity = capacity.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut left = 0;
        let mut right = capacity.len();
        let mut left_sum = 0;

        while left < right {
            let key = capacity[left.midpoint(right)];
            let middle = Self::partition(&mut capacity, left, right, key);
            let candidate_left_sum = capacity[left..middle].iter().fold(left_sum, |acc, &value| acc + value);

            if candidate_left_sum < total_apples {
                left_sum = candidate_left_sum + key;
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        Self::minimum_boxes(apple, capacity)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
