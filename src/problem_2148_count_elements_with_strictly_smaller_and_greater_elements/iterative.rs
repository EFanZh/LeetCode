pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let mut iter = nums.iter().copied();
        let mut min = iter.next().unwrap();
        let mut min_count = 1;
        let mut max = min;
        let mut max_count = 1;

        for num in iter {
            match num.cmp(&min) {
                Ordering::Less => {
                    min = num;
                    min_count = 1;
                }
                Ordering::Equal => min_count += 1,
                Ordering::Greater => {}
            }

            match num.cmp(&max) {
                Ordering::Less => {}
                Ordering::Equal => max_count += 1,
                Ordering::Greater => {
                    max = num;
                    max_count = 1;
                }
            }
        }

        if min == max {
            0
        } else {
            nums.len() as i32 - min_count - max_count
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_elements(nums: Vec<i32>) -> i32 {
        Self::count_elements(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
