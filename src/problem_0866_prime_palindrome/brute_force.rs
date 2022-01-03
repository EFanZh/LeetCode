pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://leetcode.com/problems/prime-palindrome/discuss/146798/JavaC%2B%2BPython-All-Even-Length-Palindrome-are-Divisible-by-11>.

impl Solution {
    fn count_digits(mut n: u32) -> u32 {
        let mut result = 0;

        while n != 0 {
            n /= 10;
            result += 1;
        }

        result
    }

    fn make_palindrome(mut seed: u32) -> u32 {
        let mut high = seed;
        let mut low = 0;

        loop {
            seed /= 10;

            if seed == 0 {
                break;
            }

            high *= 10;
            low = low * 10 + seed % 10;
        }

        high + low
    }

    fn smallest_palindrome_seed(n: u32) -> u32 {
        let num_digits = Self::count_digits(n);
        let low_base = u32::pow(10, num_digits / 2);

        if num_digits % 2 == 0 {
            low_base
        } else {
            let seed = n / low_base;
            let palindrome = Self::make_palindrome(seed);

            if palindrome < n {
                seed + 1
            } else {
                seed
            }
        }
    }

    fn is_prime(n: u32) -> bool {
        let end = f64::from(n).sqrt() as u32;

        for x in 2..=end {
            if n % x == 0 {
                return false;
            }
        }

        true
    }

    pub fn prime_palindrome(n: i32) -> i32 {
        let n = n as u32;

        match n {
            0..=2 => 2,
            3 => 3,
            4..=5 => 5,
            6..=7 => 7,
            8..=11 => 11,
            12..=101 => 101,
            _ => {
                let mut seed = Self::smallest_palindrome_seed(n);

                loop {
                    let palindrome = Self::make_palindrome(seed);

                    if Self::is_prime(palindrome) {
                        return palindrome as _;
                    }

                    seed += 1;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn prime_palindrome(n: i32) -> i32 {
        Self::prime_palindrome(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
