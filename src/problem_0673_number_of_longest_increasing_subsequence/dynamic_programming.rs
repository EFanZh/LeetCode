pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut cache = Vec::<(i32, i32)>::with_capacity(nums.len());
        let mut result_length = 0;
        let mut result_count = 1;

        for &num in &nums {
            let mut max_length = 1;
            let mut max_count = 1;

            for (&(length, count), &x) in cache.iter().zip(&nums) {
                if x < num {
                    match (length + 1).cmp(&max_length) {
                        Ordering::Less => {}
                        Ordering::Equal => max_count += count,
                        Ordering::Greater => {
                            max_length = length + 1;
                            max_count = count;
                        }
                    }
                }
            }

            cache.push((max_length, max_count));

            match max_length.cmp(&result_length) {
                Ordering::Less => {}
                Ordering::Equal => result_count += max_count,
                Ordering::Greater => {
                    result_length = max_length;
                    result_count = max_count;
                }
            }
        }

        result_count
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        Self::find_number_of_lis(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
