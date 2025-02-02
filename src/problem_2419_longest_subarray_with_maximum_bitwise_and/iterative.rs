pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_value = 0;
        let mut max_value_length = 0;
        let mut length = 0;
        let mut iter = nums.iter().map(|&x| x as u32);

        'outer: loop {
            if let Some(num) = iter.next() {
                match num.cmp(&max_value) {
                    Ordering::Less => {
                        max_value_length = max_value_length.max(length);

                        loop {
                            if let Some(num) = iter.next() {
                                match num.cmp(&max_value) {
                                    Ordering::Less => continue,
                                    Ordering::Equal => {}
                                    Ordering::Greater => {
                                        max_value = num;
                                        max_value_length = 1;
                                    }
                                }

                                length = 1;

                                break;
                            }

                            break 'outer;
                        }
                    }
                    Ordering::Equal => length += 1,
                    Ordering::Greater => {
                        max_value = num;
                        max_value_length = 1;
                        length = 1;
                    }
                }
            } else {
                max_value_length = max_value_length.max(length);

                break;
            }
        }

        max_value_length
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_subarray(nums: Vec<i32>) -> i32 {
        Self::longest_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
