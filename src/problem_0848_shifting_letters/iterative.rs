pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut s = s.into_bytes();
        let mut shift = 0;

        for (c, &extra_shift) in s.iter_mut().zip(&shifts).rev() {
            shift = ((u32::from(shift) + extra_shift as u32) % 26) as u8;

            *c += shift;

            if *c > b'z' {
                *c -= 26;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        Self::shifting_letters(s, shifts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
