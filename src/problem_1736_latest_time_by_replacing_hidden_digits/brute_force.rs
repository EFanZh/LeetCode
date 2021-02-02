pub struct Solution;

use std::convert::TryInto;

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut time = time.into_bytes();
        let [h1, h2, _, m1, m2]: &mut [u8; 5] = time.as_mut_slice().try_into().unwrap();

        match h1 {
            b'0' | b'1' => {
                if *h2 == b'?' {
                    *h2 = b'9';
                }
            }
            b'2' => {
                if *h2 == b'?' {
                    *h2 = b'3';
                }
            }
            _ => {
                if *h2 < b'4' {
                    *h1 = b'2';
                } else if *h2 <= b'9' {
                    *h1 = b'1';
                } else {
                    *h1 = b'2';
                    *h2 = b'3';
                }
            }
        }

        if *m1 == b'?' {
            *m1 = b'5';
        }

        if *m2 == b'?' {
            *m2 = b'9';
        }

        String::from_utf8(time).unwrap()
    }
}

impl super::Solution for Solution {
    fn maximum_time(time: String) -> String {
        Self::maximum_time(time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
