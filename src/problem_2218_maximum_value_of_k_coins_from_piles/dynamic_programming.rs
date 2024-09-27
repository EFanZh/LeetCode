pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as u32 as usize;
        let mut buffer = vec![0_u32; (k + 1) * 2].into_boxed_slice();
        let (mut cache, mut temp) = buffer.split_at_mut(k + 1);

        for mut pile in piles {
            let mut sum = 0;

            pile.iter_mut().take(k + 1).for_each(|value| {
                sum += *value;
                *value = sum;
            });

            (1_usize..).zip(&mut temp[1..]).for_each(|(max_count, target)| {
                *target = cache[..max_count]
                    .iter()
                    .rev()
                    .zip(&pile)
                    .fold(cache[max_count], |max, (&prev_max, &value)| {
                        max.max(prev_max + value as u32)
                    });
            });

            mem::swap(&mut cache, &mut temp);
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::max_value_of_coins(piles, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
