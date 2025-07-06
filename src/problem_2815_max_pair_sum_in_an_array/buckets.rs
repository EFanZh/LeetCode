pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn max_digit(mut num: u16) -> u8 {
        let mut max_digit = 0;

        while num != 0 {
            max_digit = max_digit.max((num % 10) as u8);
            num /= 10;
        }

        max_digit
    }

    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut buckets = [(0_u16, 0_u16); 10];

        for num in nums {
            let num = num as u16;
            let bucket = &mut buckets[usize::from(Self::max_digit(num))];

            if num > bucket.1 {
                if num > bucket.0 {
                    bucket.1 = bucket.0;
                    bucket.0 = num;
                } else {
                    bucket.1 = num;
                }
            }
        }

        i32::from(
            buckets
                .iter()
                .filter(|&&(_, x)| x > 0)
                .map(|&(x, y)| x + y)
                .max()
                .unwrap_or(u16::MAX) as i16,
        )
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum(nums: Vec<i32>) -> i32 {
        Self::max_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
