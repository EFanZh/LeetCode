pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn rearrange_string(s: String, x: char, y: char) -> String {
        let mut s = s.into_bytes();
        let x = x as u8;
        let y = y as u8;
        let mut iter = s.iter_mut();

        while let Some(left) = iter.find(|c| **c == x) {
            if let Some(right) = iter.rfind(|c| **c == y) {
                *left = y;
                *right = x;
            } else {
                break;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rearrange_string(s: String, x: char, y: char) -> String {
        Self::rearrange_string(s, x, y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
