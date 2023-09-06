pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut iter = nums.iter().copied();

        loop {
            // Positive.

            if let Some(num) = iter.next() {
                match num.cmp(&0) {
                    Ordering::Less => loop {
                        // Negative.

                        if let Some(num) = iter.next() {
                            match num.cmp(&0) {
                                Ordering::Less => break,
                                Ordering::Equal => return 0,
                                Ordering::Greater => {}
                            }
                        } else {
                            return -1;
                        }
                    },
                    Ordering::Equal => return 0,
                    Ordering::Greater => {}
                }
            } else {
                return 1;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn array_sign(nums: Vec<i32>) -> i32 {
        Self::array_sign(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
