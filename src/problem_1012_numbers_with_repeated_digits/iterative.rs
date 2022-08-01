pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_digits(mut n: u32, buffer: &mut [u8; 10]) -> &[u8] {
        let mut start = 10;

        while n != 0 {
            start -= 1;
            buffer[start] = (n % 10) as _;
            n /= 10;
        }

        &buffer[start..]
    }

    fn arrangements(mut m: u32, n: u32) -> u32 {
        let mut result = 1;

        for _ in 0..n {
            result *= m;
            m -= 1;
        }

        result
    }

    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let n = n as u32;
        let mut buffer = [0_u8; 10];
        let digits = Self::get_digits(n, &mut buffer);
        let digits_length = digits.len() as u32;
        let mut all_uniques = 1;

        for length in 1..digits_length {
            all_uniques += 9 * Self::arrangements(9, length - 1);
        }

        let mut used = [false; 10];

        for (i, &digit) in (1..).zip(digits) {
            let start = u8::from(i == 1);
            let selections = (start..digit).filter(|&d| !used[usize::from(d)]).count() as u32;

            all_uniques += selections * Self::arrangements(10 - i, digits_length - i);

            if let used_value @ false = &mut used[usize::from(digit)] {
                *used_value = true;
            } else {
                all_uniques -= 1;

                break;
            }
        }

        (n - all_uniques) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_dup_digits_at_most_n(n: i32) -> i32 {
        Self::num_dup_digits_at_most_n(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
