pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn sort(value: &mut [u8; 4]) {
        let [a, b, c, d] = value;

        if *c < *a {
            mem::swap(a, c);
        }

        if *d < *b {
            mem::swap(b, d);
        }
    }

    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut s1 = s1.into_bytes();
        let s1 = <&mut [_; 4]>::try_from(s1.as_mut_slice()).ok().unwrap();
        let mut s2 = s2.into_bytes();
        let s2 = <&mut [_; 4]>::try_from(s2.as_mut_slice()).ok().unwrap();

        Self::sort(s1);
        Self::sort(s2);

        s1 == s2
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_be_equal(s1: String, s2: String) -> bool {
        Self::can_be_equal(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
