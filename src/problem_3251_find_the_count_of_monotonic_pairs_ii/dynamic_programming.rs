pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        const MODULUS: u32 = 1_000_000_007;

        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut iter = nums.iter();
        let mut prev = iter.next().copied().unwrap();
        let mut cache = (1..=prev + 1).collect::<Vec<_>>();
        let mut temp = Vec::new();

        for &num in iter {
            let mut sum = 0;

            temp.extend((0..=num).map(|split| {
                sum += prev
                    .checked_sub(num - split)
                    .map_or(0, |max_prev| cache[(split.min(max_prev) as usize).min(cache.len() - 1)]);

                if sum >= MODULUS {
                    sum -= MODULUS;
                }

                sum
            }));

            cache.clear();
            prev = num;
            mem::swap(&mut cache, &mut temp);
        }

        cache.last().unwrap().cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_of_pairs(nums: Vec<i32>) -> i32 {
        Self::count_of_pairs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
