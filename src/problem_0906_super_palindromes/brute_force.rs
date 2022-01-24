pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert;

impl Solution {
    fn sqrt(value: u64) -> u64 {
        #[allow(clippy::cast_precision_loss)]
        let guess = (value as f64).sqrt() as u64;

        guess
    }

    fn get_palindrome_seed(mut value: u64) -> (u64, u32) {
        let mut base = 10;
        let mut digits = 0;

        while value >= base {
            value /= 10;
            digits += 1;
            base *= 10;
        }

        let seed = value;

        while value != 0 {
            value /= 10;
            digits += 1;
        }

        (seed, digits)
    }

    fn make_candidate(mut high: u64, mut seed: u64) -> u64 {
        let mut low = 0;

        while seed != 0 {
            low = low * 10 + seed % 10;
            seed /= 10;
            high *= 10;
        }

        (high + low).checked_pow(2).unwrap_or(u64::MAX)
    }

    fn reverse(mut value: u64) -> u64 {
        let mut result = 0;

        while value != 0 {
            result = result * 10 + value % 10;
            value /= 10;
        }

        result
    }

    fn is_palindrome(value: u64) -> bool {
        Self::reverse(value) == value
    }

    fn count_palindromes(low: u64, high: u64, mut seed: u64, mut f: impl FnMut(u64) -> u64) -> i32 {
        let mut value = loop {
            let value = Self::make_candidate(f(seed), seed);

            if value < low {
                seed += 1;
            } else {
                break value;
            }
        };

        let mut result = 0;

        while value <= high {
            if Self::is_palindrome(value) {
                result += 1;
            }

            seed += 1;
            value = Self::make_candidate(f(seed), seed);
        }

        result
    }

    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let low = left.parse::<u64>().unwrap();
        let high = right.parse::<u64>().unwrap();
        let (seed, digits) = Self::get_palindrome_seed(Self::sqrt(low));

        let (seed_1, seed_2) = if digits % 2 == 0 {
            (u64::pow(10, digits / 2), seed)
        } else {
            (seed, u64::pow(10, digits / 2))
        };

        let odd = Self::count_palindromes(low, high, seed_1, |x| x / 10);
        let even = Self::count_palindromes(low, high, seed_2, convert::identity);

        odd + even
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn superpalindromes_in_range(left: String, right: String) -> i32 {
        Self::superpalindromes_in_range(left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
