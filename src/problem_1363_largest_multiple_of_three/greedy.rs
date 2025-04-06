pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    fn helper(nums: Vec<i32>) -> String {
        let mut min_0 = u32::MAX;
        let mut min_1 = (u32::MAX, u32::MAX);
        let mut min_2 = (u32::MAX, u32::MAX);
        let mut sum = 0;
        let mut counts = [0; 10];

        for &value in &nums {
            let value = value as u32;
            let remainder = value as u8 % 3;

            match remainder {
                0 => min_0 = min_0.min(value),
                1 => {
                    min_1 = if value <= min_1.0 {
                        (value, min_1.0)
                    } else {
                        (min_1.0, min_1.1.min(value))
                    }
                }
                _ => {
                    min_2 = if value <= min_2.0 {
                        (value, min_2.0)
                    } else {
                        (min_2.0, min_2.1.min(value))
                    }
                }
            }

            counts[value as usize] += 1;
            sum += u32::from(remainder);
        }

        let mut result = String::new();

        let removed = match sum % 3 {
            0 => 0,
            1 => {
                if min_1.0 == u32::MAX {
                    counts[min_2.0 as usize] -= 1;
                    counts[min_2.1 as usize] -= 1;

                    2
                } else {
                    counts[min_1.0 as usize] -= 1;

                    1
                }
            }
            _ => {
                if min_2.0 == u32::MAX {
                    counts[min_1.0 as usize] -= 1;
                    counts[min_1.1 as usize] -= 1;

                    2
                } else {
                    counts[min_2.0 as usize] -= 1;

                    1
                }
            }
        };

        if removed != nums.len() {
            if counts[1..].iter().all(|&count| count == 0) {
                result.push('0');
            } else {
                result.reserve(nums.len() - removed);

                for (c, count) in (b'0'..=b'9').zip(counts).rev() {
                    result.extend(iter::repeat_n(c, count).map(char::from));
                }
            }
        }

        result
    }

    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        Self::helper(digits)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        Self::largest_multiple_of_three(digits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
