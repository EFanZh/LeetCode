pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as u32;
        let k = k as u32 - 1;
        let mut result = String::new();
        let first_digit = (k >> (n - 1)) as u8;

        if first_digit < 3 {
            result.reserve(n as _);
            result.push(char::from(b'a' + first_digit));

            let mut prev_digit = first_digit;
            let mut probe = 1 << (n - 1);

            loop {
                probe >>= 1;

                if probe == 0 {
                    break;
                }

                let digit = match prev_digit * 2 + u8::from((k & probe) != 0) {
                    1 | 3 => 2,
                    2 | 4 => 0,
                    _ => 1,
                };

                result.push(char::from(b'a' + digit));
                prev_digit = digit;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_happy_string(n: i32, k: i32) -> String {
        Self::get_happy_string(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
