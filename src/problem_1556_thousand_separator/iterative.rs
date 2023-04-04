pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str;

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let mut n = n as u32;
        let mut buffer;

        if n == 0 {
            "0"
        } else {
            buffer = [0_u8; 14];

            let mut i = 14;

            loop {
                i -= 1;
                buffer[i] = b'0' + (n % 10) as u8;
                n /= 10;

                if n == 0 {
                    break;
                }

                i -= 1;
                buffer[i] = b'0' + (n % 10) as u8;
                n /= 10;

                if n == 0 {
                    break;
                }

                i -= 1;
                buffer[i] = b'0' + (n % 10) as u8;
                n /= 10;

                if n == 0 {
                    break;
                }

                i -= 1;
                buffer[i] = b'.';
            }

            str::from_utf8(&buffer[i..]).unwrap()
        }
        .to_string()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn thousand_separator(n: i32) -> String {
        Self::thousand_separator(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
