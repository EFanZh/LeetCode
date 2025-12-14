pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let mut s = s;
        let k = k as usize;

        if let Some(n) = NonZero::new(s.len()).filter(|&n| k % n != 0) {
            let mut bytes = s.into_bytes();

            bytes.rotate_left(k % n);

            s = String::from_utf8(bytes).unwrap();
        }

        s
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_encrypted_string(s: String, k: i32) -> String {
        Self::get_encrypted_string(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
