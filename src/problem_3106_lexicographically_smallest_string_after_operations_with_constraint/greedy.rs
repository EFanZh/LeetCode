pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_smallest_string(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        let mut k = k.cast_unsigned();

        for c in &mut s {
            let diff = u32::from(u8::min(*c - b'a', b'z' + 1 - *c));

            if diff <= k {
                *c = b'a';
                k -= diff;
            } else {
                *c -= k as u8;

                break;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_smallest_string(s: String, k: i32) -> String {
        Self::get_smallest_string(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
