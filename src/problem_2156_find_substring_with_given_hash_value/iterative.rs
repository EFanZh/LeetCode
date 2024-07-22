pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU64;

impl Solution {
    fn pow_mod(mut base: u64, mut exponent: u32, modulus: NonZeroU64) -> u64 {
        let mut result = 1;

        while exponent != 0 {
            if exponent & 1 != 0 {
                result = (result * base) % modulus;
            }

            base = (base * base) % modulus;
            exponent >>= 1;
        }

        result
    }

    pub fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
        let slice = s.as_bytes();
        let power = u64::from(power as u32);
        let modulo = NonZeroU64::new(u64::from(modulo as u32)).unwrap();
        let k = k as u32 as usize;
        let hash_value = hash_value as u32;
        let n = slice.len();
        let mut current_hash = 0;

        for &c in slice[n - k..].iter().rev() {
            current_hash = (current_hash * power + u64::from(c - (b'a' - 1))) % modulo;
        }

        let mut iter = slice.iter().zip(&slice[k..]);
        let mut i = n - k;
        let mut start = 0;
        let base = Self::pow_mod(power, k as u32 - 1, modulo);

        loop {
            if current_hash as u32 == hash_value {
                start = i;
            }

            if let Some((&new, &old)) = iter.next_back() {
                let (new, old) = (new - (b'a' - 1), old - (b'a' - 1));
                let old_hash = (base * u64::from(old)) % modulo;

                current_hash = ((current_hash + modulo.get() - old_hash) * power + u64::from(new)) % modulo;
                i -= 1;
            } else {
                break;
            }
        }

        let mut result = s;

        result.truncate(start + k);
        result.replace_range(..start, "");

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
        Self::sub_str_hash(s, power, modulo, k, hash_value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
