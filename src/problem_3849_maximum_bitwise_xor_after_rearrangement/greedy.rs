pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_xor(s: String, t: String) -> String {
        let mut result = s.into_bytes();
        let mut ones = t.bytes().map(|c| u32::from(c & 1)).sum::<u32>();
        let mut zeros = t.len() as u32 - ones;
        let mut iter = result.iter_mut();

        while let Some(c) = iter.next() {
            if *c == b'0' {
                if ones == 0 {
                    break;
                }

                *c = b'1';
                ones -= 1;
            } else {
                if zeros == 0 {
                    *c ^= 1;
                    iter.for_each(|c| *c ^= 1);

                    break;
                }

                zeros -= 1;
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_xor(s: String, t: String) -> String {
        Self::maximum_xor(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
