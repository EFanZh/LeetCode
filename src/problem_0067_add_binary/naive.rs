pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        if b.len() < a.len() {
            mem::swap(&mut a, &mut b);
        }

        let a_len = a.len();
        let mut b = b.into_bytes();
        let split = b.len() - a_len;
        let (b_left, b_right) = b.split_at_mut(split);
        let mut carry = false;

        for (target, source) in b_right.iter_mut().zip(a.into_bytes()).rev() {
            *target += source - b'0' + u8::from(carry);
            carry = *target > b'1';

            if carry {
                *target -= 2;
            }
        }

        if carry {
            for target in b_left.iter_mut().rev() {
                if *target == b'0' {
                    *target = b'1';

                    return String::from_utf8(b).unwrap();
                }

                *target = b'0';
            }

            b.insert(0, b'1');
        }

        String::from_utf8(b).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_binary(a: String, b: String) -> String {
        Self::add_binary(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
