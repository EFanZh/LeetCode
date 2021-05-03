pub struct Solution;

impl Solution {
    fn reverse(mut n: u64) -> u64 {
        let mut result = 0;

        loop {
            result += n % 10;
            n /= 10;

            if n == 0 {
                break;
            }

            result *= 10;
        }

        result
    }

    pub fn largest_palindrome(n: i32) -> i32 {
        let n = n as u32;

        for base in (u64::pow(10, n - 1)..u64::pow(10, n)).rev() {
            let num = base * u64::pow(10, n) + Self::reverse(base);
            let mut x = (num + (u64::pow(10, n) - 2)) / (u64::pow(10, n) - 1);

            loop {
                let y = num / x;

                if y < x {
                    break;
                }

                if num % x == 0 {
                    return (num % 1337) as _;
                }

                x += 1;
            }
        }

        9
    }
}

impl super::Solution for Solution {
    fn largest_palindrome(n: i32) -> i32 {
        Self::largest_palindrome(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
