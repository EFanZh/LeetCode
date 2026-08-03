pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut counts = [0; 10];

        (0..).zip(nums).for_each(|(i, num)| {
            let mut num = num.cast_unsigned();
            let last = num % 10;

            while num >= 10 {
                num /= 10;
            }

            result += match last {
                1 => i,
                2 | 4 | 8 => counts[1] + counts[3] + counts[5] + counts[7] + counts[9],
                3 | 9 => i - counts[3] - counts[6] - counts[9],
                5 => i - counts[5],
                6 => counts[1] + counts[5] + counts[7],
                _ => i - counts[7],
            };

            counts[num as usize] += 1;
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        Self::count_beautiful_pairs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
