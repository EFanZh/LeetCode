pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn fingerprint(s: &str) -> [u32; 52] {
        let mut result = [0; 52];
        let mut offset = usize::from(b'a').wrapping_neg();
        let x = 26_usize.wrapping_sub(usize::from(b'a') * 2);

        s.bytes().for_each(|c| {
            result[usize::from(c).wrapping_add(offset)] += 1;
            offset = x.wrapping_sub(offset);
        });

        result
    }

    pub fn check_strings(s1: String, s2: String) -> bool {
        Self::fingerprint(&s1) == Self::fingerprint(&s2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_strings(s1: String, s2: String) -> bool {
        Self::check_strings(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
