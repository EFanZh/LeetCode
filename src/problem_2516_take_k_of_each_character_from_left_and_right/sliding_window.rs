pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let s = s.into_bytes();
        let k = k as u32;
        let mut a_count = 0_u32;
        let mut b_count = 0_u32;
        let mut c_count = 0_u32;

        for &c in &s {
            match c {
                b'a' => a_count += 1,
                b'b' => b_count += 1,
                _ => c_count += 1,
            }
        }

        if a_count < k || b_count < k || c_count < k {
            return -1;
        }

        let mut start = 0;

        for &c in &s {
            match c {
                b'a' => a_count -= 1,
                b'b' => b_count -= 1,
                _ => c_count -= 1,
            }

            if a_count < k || b_count < k || c_count < k {
                let old = s[start];

                match old {
                    b'a' => a_count += 1,
                    b'b' => b_count += 1,
                    _ => c_count += 1,
                }

                start += 1;
            }
        }

        start as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn take_characters(s: String, k: i32) -> i32 {
        Self::take_characters(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
