pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let k = k as u32 as usize;
        let mut s = s.into_bytes();

        for c in &mut s {
            *c -= b'0';
        }

        let mut temp = Vec::new();
        let mut buffer = [0; 5];

        while s.len() > k {
            for group in s.chunks(k) {
                let mut sum = 0;

                for &digit in group {
                    sum += u16::from(digit);
                }

                let mut start = 5;

                while sum != 0 {
                    start -= 1;
                    buffer[start] = (sum % 10) as _;
                    sum /= 10;
                }

                if start == 5 {
                    temp.push(0);
                } else {
                    temp.extend(&buffer[start..]);
                }
            }

            mem::swap(&mut s, &mut temp);

            temp.clear();
        }

        for c in &mut s {
            *c += b'0';
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn digit_sum(s: String, k: i32) -> String {
        Self::digit_sum(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
