pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn ceil_isqrt(x: u32) -> u32 {
        let result = x.isqrt();

        result + u32::from(result * result != x)
    }

    fn sieve_of_eratosthenes(max: u32) -> Box<[usize]> {
        let mut states = vec![0; (max / usize::BITS + 1) as usize].into_boxed_slice();

        states[0] = 3; // 0 and 1 are not primes.

        for factor in 2..=max.isqrt() {
            let mut index = factor;

            if states[(index / usize::BITS) as usize] & (1 << (index % usize::BITS)) == 0 {
                index = factor * factor;

                while let Some(state) = states.get_mut((index / usize::BITS) as usize) {
                    *state |= 1 << (index % usize::BITS);
                    index += factor;
                }
            }
        }

        states
    }

    pub fn non_special_count(l: i32, r: i32) -> i32 {
        let left = l.cast_unsigned();
        let right = r.cast_unsigned();
        let ceil_sqrt_left = Self::ceil_isqrt(left);
        let floor_sqrt_right = right.isqrt();
        let primes = Self::sieve_of_eratosthenes(floor_sqrt_right);
        let left_bucket = ceil_sqrt_left / usize::BITS;
        let left_bit = ceil_sqrt_left % usize::BITS;
        let right_bucket = floor_sqrt_right / usize::BITS;
        let right_bit = floor_sqrt_right % usize::BITS;

        let non_primes = if left_bucket == right_bucket {
            (primes[left_bucket as usize] & ((usize::unbounded_shl(1, right_bit + 1 - left_bit) - 1) << left_bit))
                .count_ones()
        } else {
            primes[left_bucket as usize + 1..right_bucket as usize].iter().fold(
                (primes[left_bucket as usize] >> left_bit).count_ones()
                    + (primes[right_bucket as usize] << (usize::BITS - 1 - right_bit)).count_ones(),
                |sum, &state| sum + state.count_ones(),
            )
        };

        let primes = floor_sqrt_right + 1 - ceil_sqrt_left - non_primes;

        (right + 1 - left - primes).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn non_special_count(l: i32, r: i32) -> i32 {
        Self::non_special_count(l, r)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
