pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    const MODULUS: u32 = 1_000_000_007;

    fn add_mod(lhs: &mut u32, rhs: u32) {
        let candidate = *lhs + rhs;

        *lhs = candidate.checked_sub(Self::MODULUS).unwrap_or(candidate);
    }

    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let target = target as usize;
        let mut cache = vec![0_u32; (target + 1) * 2];
        let (mut cache, mut temp) = cache.split_at_mut(target + 1);

        cache[0] = 1;

        for dices in 1..=n {
            let mut total = 0;

            for (i, count) in (dices..).zip(&mut temp[dices..=(k * dices).min(target)]) {
                Self::add_mod(&mut total, cache.get(i.wrapping_sub(1)).copied().unwrap_or(0));

                Self::add_mod(
                    &mut total,
                    Self::MODULUS - cache.get(i.wrapping_sub(k + 1)).copied().unwrap_or(0),
                );

                *count = total;
            }

            cache[dices - 1] = 0;
            cache[dices] = 0;

            mem::swap(&mut cache, &mut temp);
        }

        cache[target] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        Self::num_rolls_to_target(n, k, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
