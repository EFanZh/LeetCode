pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut s = s.into_bytes();
        let mut iter = s.iter_mut();
        let mut left = 0;
        let mut middle = iter.next().unwrap();

        for right in iter {
            if *middle == b'?' {
                *middle = b'a'
                    + if *right == b'?' {
                        u8::from(left == b'a')
                    } else {
                        match left {
                            b'a' => 1 + u8::from(*right == b'b'),
                            b'b' => u8::from(*right == b'a') * 2,
                            _ => u8::from(*right == b'a'),
                        }
                    };
            }

            left = *middle;
            middle = right;
        }

        if *middle == b'?' {
            *middle = b'a' + u8::from(left == b'a');
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn modify_string(s: String) -> String {
        Self::modify_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
