pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn exp_mod(mut base: u64, mut exponent: u32) -> u64 {
        let mut result = 1;

        loop {
            if exponent & 1 != 0 {
                result = (result * base) % Self::MODULUS;
            }

            exponent >>= 1;

            if exponent == 0 {
                break;
            }

            base = (base * base) % Self::MODULUS;
        }

        result
    }

    fn mod_inverse(x: u64) -> u64 {
        Self::exp_mod(x, Self::MODULUS as u32 - 2)
    }

    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        let n = n as u32 as usize;
        let max_value = max_value as u32 as usize;
        let max_unique_values = n.min((usize::BITS - max_value.leading_zeros()) as _);
        let mut cache = vec![0_u32; max_value * max_unique_values].into_boxed_slice();
        let mut iter = cache.chunks_exact_mut(max_value);
        let mut prev_row = iter.next().unwrap();
        let mut start = 1;

        prev_row.fill(1);

        iter.for_each(|current_row| {
            (start..).zip(&prev_row[start - 1..]).for_each(|(value, &count)| {
                if count != 0 {
                    let mut index = value * 2 - 1;

                    while let Some(target) = current_row.get_mut(index) {
                        *target += count;

                        index += value;
                    }
                }
            });

            prev_row = current_row;
            start <<= 1;
        });

        let mut factorials = vec![0_u32; n].into_boxed_slice();
        let mut factorial = 1;

        (1..).zip(&mut *factorials).for_each(|(factor, target)| {
            *target = factorial as _;
            factorial = (factorial * factor) % Self::MODULUS;
        });

        let top = u64::from(*factorials.last().unwrap());

        cache
            .chunks_exact(max_value)
            .enumerate()
            .fold(0, |mut result, (k, row)| {
                let count = row[(1 << k) - 1..].iter().sum::<u32>();

                let combinations = (top
                    * Self::mod_inverse((u64::from(factorials[k]) * u64::from(factorials[n - 1 - k])) % Self::MODULUS))
                    % Self::MODULUS;

                result += (combinations * u64::from(count) % Self::MODULUS) as u32;

                result.checked_sub(Self::MODULUS as _).unwrap_or(result)
            }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        Self::ideal_arrays(n, max_value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
