pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut iter = nums.iter().copied();

        // Initial.

        let mut prev = iter.next().unwrap();

        let mut is_increasing = loop {
            if let Some(num) = iter.next() {
                match num.cmp(&prev) {
                    Ordering::Less => break false,
                    Ordering::Equal => {}
                    Ordering::Greater => break true,
                }
            } else {
                return result;
            }
        };

        for num in nums {
            let new_is_increasing = match num.cmp(&prev) {
                Ordering::Less => false,
                Ordering::Equal => continue,
                Ordering::Greater => true,
            };

            result += i32::from(is_increasing != new_is_increasing);

            is_increasing = new_is_increasing;
            prev = num;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_hill_valley(nums: Vec<i32>) -> i32 {
        Self::count_hill_valley(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
