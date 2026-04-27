pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn assign_add(target: &mut u32, value: u32) {
        const MODULUS: u32 = 1_000_000_007;

        *target += value;

        if *target >= MODULUS {
            *target -= MODULUS;
        }
    }

    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        let mut result = 0;

        for num in nums.into_iter().map(i32::cast_unsigned) {
            if num.is_multiple_of(2)
                && let Some(&(_, count_2)) = cache.get(&(num / 2))
            {
                Self::assign_add(&mut result, count_2);
            }

            let count_2 = cache.get(&(num * 2)).map_or(0, |&(count_1, _)| count_1);

            cache
                .entry(num)
                .and_modify(|entry| {
                    entry.0 += 1;
                    Self::assign_add(&mut entry.1, count_2);
                })
                .or_insert((1, count_2));
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn special_triplets(nums: Vec<i32>) -> i32 {
        Self::special_triplets(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
