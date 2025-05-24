pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn mod_inverse(mut base: u64) -> u64 {
        let mut result = 1;
        let mut exponent = Self::MODULUS - 2;

        loop {
            if exponent & 1 != 0 {
                result = (result * base) % Self::MODULUS;

                if exponent == 1 {
                    break;
                }
            }

            exponent >>= 1;
            base = (base * base) % Self::MODULUS;
        }

        result
    }

    fn count_word_anagrams(factorials: &mut Vec<u32>, counts: &[u32; 26]) -> u64 {
        let n = counts.iter().sum::<u32>() as usize;
        let current_factorials_len = factorials.len();

        if current_factorials_len <= n {
            let mut prev = u64::from(*factorials.last().unwrap());

            factorials.extend((current_factorials_len..=n).map(|i| {
                prev = (prev * i as u64) % Self::MODULUS;

                prev as u32
            }));
        }

        let denominator = counts.iter().fold(1, |denominator, &count| {
            (denominator * u64::from(factorials[count as usize])) % Self::MODULUS
        });

        (u64::from(factorials[n]) * Self::mod_inverse(denominator)) % Self::MODULUS
    }

    pub fn count_anagrams(s: String) -> i32 {
        let mut counts = [0_u32; 26];
        let mut factorials = vec![1];
        let mut result = 1;

        for c in s.into_bytes() {
            if let Some(count) = counts.get_mut(usize::from(c).wrapping_sub(usize::from(b'a'))) {
                *count += 1;
            } else {
                result = (result * Self::count_word_anagrams(&mut factorials, &counts)) % Self::MODULUS;

                counts.fill(0);
            }
        }

        ((result * Self::count_word_anagrams(&mut factorials, &counts)) % Self::MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_anagrams(s: String) -> i32 {
        Self::count_anagrams(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
