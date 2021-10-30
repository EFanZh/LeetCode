pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn reverse(mut value: u64) -> u64 {
        let mut result = 0;

        while value != 0 {
            result = result * 10 + value % 10;
            value /= 10;
        }

        result
    }

    fn shr(value: u64, n: u32) -> u64 {
        value / u64::pow(10, n)
    }

    fn shl(value: u64, n: u32) -> u64 {
        value * u64::pow(10, n)
    }

    fn is_powers_of_10(mut value: u64) -> bool {
        while value != 1 {
            if value % 10 != 0 {
                return false;
            }

            value /= 10;
        }

        true
    }

    fn helper(n: u64, len: u32) -> u64 {
        let is_len_even = len % 2 == 0;
        let half_len = len / 2;
        let base = Self::shr(n, half_len);
        let candidate = Self::shl(base, half_len) + Self::reverse(if is_len_even { base } else { base / 10 });

        let get_low = || {
            if Self::is_powers_of_10(base) {
                u64::pow(10, len - 1) - 1
            } else if is_len_even {
                Self::shl(base - 1, half_len) + Self::reverse(base - 1)
            } else {
                Self::shl(base - 1, half_len) + Self::reverse((base - 1) / 10)
            }
        };

        let get_high = || {
            if Self::is_powers_of_10(base + 1) {
                u64::pow(10, len) + 1
            } else if is_len_even {
                Self::shl(base + 1, half_len) + Self::reverse(base + 1)
            } else {
                Self::shl(base + 1, half_len) + Self::reverse((base + 1) / 10)
            }
        };

        let (low, high) = match candidate.cmp(&n) {
            Ordering::Less => (candidate, get_high()),
            Ordering::Equal => (get_low(), get_high()),
            Ordering::Greater => (get_low(), candidate),
        };

        if high - n < n - low {
            high
        } else {
            low
        }
    }

    pub fn nearest_palindromic(n: String) -> String {
        use std::fmt::Write;

        let mut n = n;
        let result = Self::helper(n.parse().unwrap(), n.len() as _);

        n.clear();

        write!(&mut n, "{}", result).unwrap();

        n
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nearest_palindromic(n: String) -> String {
        Self::nearest_palindromic(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
