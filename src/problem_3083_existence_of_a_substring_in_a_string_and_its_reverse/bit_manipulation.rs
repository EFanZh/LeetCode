pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let mut cache = [0_u64; 11];

        for c in s.as_bytes().windows(2) {
            let [c_0, c_1] = c.try_into().ok().unwrap();

            if c_0 == c_1 {
                return true;
            }

            let key = (u16::from(c_0) - u16::from(b'a')) * 26 + u16::from(c_1) - u16::from(b'a');

            cache[usize::from(key / 64)] |= 1 << (key % 64);
        }

        s.as_bytes().windows(2).any(|c| {
            let [c_0, c_1] = c.try_into().ok().unwrap();
            let key = (u16::from(c_1) - u16::from(b'a')) * 26 + u16::from(c_0) - u16::from(b'a');

            (cache[usize::from(key / 64)] & 1 << (key % 64)) != 0
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_substring_present(s: String) -> bool {
        Self::is_substring_present(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
