pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut less_count = 0;
        let mut equal_count = 0;

        for &num in &nums {
            less_count += usize::from(num < pivot);
            equal_count += usize::from(num == pivot);
        }

        let mut result = vec![0; nums.len()];
        let (left, rest) = result.split_at_mut(less_count);
        let (middle, right) = rest.split_at_mut(equal_count);
        let mut left_iter = left.iter_mut();
        let mut right_iter = right.iter_mut();

        for num in nums {
            let iter = match num.cmp(&pivot) {
                Ordering::Less => left_iter.by_ref(),
                Ordering::Equal => continue,
                Ordering::Greater => right_iter.by_ref(),
            };

            *iter.next().unwrap() = num;
        }

        middle.fill(pivot);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        Self::pivot_array(nums, pivot)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
