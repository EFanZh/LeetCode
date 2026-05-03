pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn write_integer<const RADIX: u32>(mut n: u32, result: &mut Vec<u8>) {
        let start = result.len();

        while n != 0 {
            let digit = (n % RADIX) as u8;

            n /= RADIX;

            result.push(if digit < 10 { b'0' } else { b'A' - 10 } + digit);
        }

        result[start..].reverse();
    }

    pub fn concat_hex36(n: i32) -> String {
        let n = n.cast_unsigned();
        let n_squared = n * n;
        let n_cubed = n_squared * n;
        let mut result = Vec::new();

        Self::write_integer::<16>(n_squared, &mut result);
        Self::write_integer::<36>(n_cubed, &mut result);

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn concat_hex36(n: i32) -> String {
        Self::concat_hex36(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
