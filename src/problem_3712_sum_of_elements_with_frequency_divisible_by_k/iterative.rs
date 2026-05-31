pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let k = NonZero::new(k.cast_unsigned()).unwrap();
        let mut counts = [0_u32; 100];

        for num in nums {
            counts[num.cast_unsigned() as usize - 1] += 1;
        }

        let mut result = 0;

        (1..).zip(&counts).for_each(|(i, &count)| {
            if count % k == 0 {
                result += i * count;
            }
        });

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        Self::sum_divisible_by_k(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
