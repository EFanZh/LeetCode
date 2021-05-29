pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn increasing(mut prev: i32, nums: &mut impl Iterator<Item = i32>, result: &mut i32) {
        while let Some(num) = nums.next() {
            if num < prev {
                *result += 1;

                return Self::decreasing(num, nums, result);
            }

            prev = num;
        }
    }

    fn decreasing(mut prev: i32, nums: &mut impl Iterator<Item = i32>, result: &mut i32) {
        while let Some(num) = nums.next() {
            if num > prev {
                *result += 1;

                return Self::increasing(num, nums, result);
            }

            prev = num;
        }
    }

    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut iter = nums.into_iter();

        if let Some(first) = iter.next() {
            loop {
                if let Some(num) = iter.next() {
                    match num.cmp(&first) {
                        Ordering::Less => {
                            result = 2;

                            Self::decreasing(num, &mut iter, &mut result);

                            break;
                        }
                        Ordering::Equal => {}
                        Ordering::Greater => {
                            result = 2;

                            Self::increasing(num, &mut iter, &mut result);

                            break;
                        }
                    }
                } else {
                    result = 1;

                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        Self::wiggle_max_length(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
