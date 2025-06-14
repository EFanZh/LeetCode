pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_string(s: String) -> String {
        let mut s = s.into_bytes();
        let mut iter = s.iter().copied();

        if let Some(start) = iter.position(|x| x != b'a') {
            let total_length = iter.len();
            let length = iter.position(|x| x == b'a').unwrap_or(total_length) + 1;

            s[start..start + length].iter_mut().for_each(|c| *c -= 1);
        } else {
            *s.last_mut().unwrap() = b'z';
        }

        String::from_utf8(s).ok().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_string(s: String) -> String {
        Self::smallest_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
