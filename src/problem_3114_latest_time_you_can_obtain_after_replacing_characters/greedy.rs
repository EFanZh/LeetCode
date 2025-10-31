pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut s = s.into_bytes();
        let [h0, h1, _, m0, m1] = <&mut [_; 5]>::try_from(s.as_mut_slice()).ok().unwrap();

        if *h0 == b'?' {
            if *h1 == b'?' {
                *h0 = b'1';
                *h1 = b'1';
            } else {
                *h0 = b'0' + u8::from(*h1 < b'2');
            }
        } else if *h1 == b'?' {
            *h1 = b'9' - (*h0 - b'0') * 8;
        }

        if m0 == &b'?' {
            *m0 = b'5';
        }

        if m1 == &b'?' {
            *m1 = b'9';
        }

        String::from_utf8(s).ok().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_latest_time(s: String) -> String {
        Self::find_latest_time(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
