pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut range = HashMap::with_capacity(nums.len());
        let mut max_frequency = 0;
        let mut min_range = 0;

        for (i, num) in (0..).zip(nums) {
            let (first, last, count) = *range
                .entry(num)
                .and_modify(|(_, last, count)| {
                    *last = i;
                    *count += 1;
                })
                .or_insert((i, i, 1));

            match count.cmp(&max_frequency) {
                Ordering::Less => {}
                Ordering::Equal => min_range = min_range.min(last - first + 1),
                Ordering::Greater => {
                    max_frequency = count;
                    min_range = last - first + 1;
                }
            }
        }

        min_range
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        Self::find_shortest_sub_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
