pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut max_frequency = 0;
        let mut max_frequency_sum = 0;
        let mut frequencies = [0_u8; 100];

        for num in nums {
            let frequency = &mut frequencies[num.cast_unsigned() as usize - 1];

            *frequency += 1;

            let frequency = u32::from(*frequency);

            match frequency.cmp(&max_frequency) {
                Ordering::Less => {}
                Ordering::Equal => max_frequency_sum += max_frequency,
                Ordering::Greater => {
                    max_frequency = frequency;
                    max_frequency_sum = max_frequency;
                }
            }
        }

        max_frequency_sum.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        Self::max_frequency_elements(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
