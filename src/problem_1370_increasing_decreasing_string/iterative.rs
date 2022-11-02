pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut counts = [0_u16; 26];

        for c in s.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut result = s.into_bytes();
        let mut iter = result.iter_mut();
        let mut slot = iter.next().unwrap();

        'outer: loop {
            for (c, count) in (b'a'..).zip(&mut counts) {
                if *count != 0 {
                    *count -= 1;

                    *slot = c;

                    if let Some(next_slot) = iter.next() {
                        slot = next_slot;
                    } else {
                        break 'outer;
                    }
                }
            }

            for (c, count) in (b'a'..=b'z').zip(counts.iter_mut()).rev() {
                if *count != 0 {
                    *count -= 1;

                    *slot = c;

                    if let Some(next_slot) = iter.next() {
                        slot = next_slot;
                    } else {
                        break 'outer;
                    }
                }
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_string(s: String) -> String {
        Self::sort_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
