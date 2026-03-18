pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_valid_pair(s: String) -> String {
        let mut counts = [0_u8; 10];

        for c in s.bytes() {
            counts[usize::from(c) - usize::from(b'0')] += 1;
        }

        let mut s = s;

        if let Some(i) = s.as_bytes().windows(2).position(|pair| {
            let [x, y] = pair.try_into().unwrap();

            x != y
                && x - b'0' == counts[usize::from(x) - usize::from(b'0')]
                && y - b'0' == counts[usize::from(y) - usize::from(b'0')]
        }) {
            s.truncate(i + 2);
            s.replace_range(..i, "");
        } else {
            s.clear();
        }

        s
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_valid_pair(s: String) -> String {
        Self::find_valid_pair(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
