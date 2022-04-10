pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn write_int(target: &mut [char], index: usize, mut value: i32) -> usize {
        let mut i = index;

        loop {
            target[i] = char::from(b'0' + (value % 10) as u8);
            i += 1;
            value /= 10;

            if value == 0 {
                break;
            }
        }

        target[index..i].reverse();

        i
    }

    #[allow(clippy::ptr_arg)] // Expected.
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if let Some(mut i) = chars
            .get(1..)
            .and_then(|rest| chars.iter().zip(rest).position(|(x, y)| x == y))
        {
            let mut prev = chars[i];

            i += 1;

            let mut count = 1;
            let mut j = i;

            while let Some(&c) = chars.get(j) {
                j += 1;

                if c == prev {
                    count += 1;
                } else {
                    if count != 1 {
                        i = Self::write_int(chars, i, count);
                    }

                    prev = c;
                    chars[i] = c;
                    i += 1;
                    count = 1;
                }
            }

            if count == 1 {
                i as _
            } else {
                Self::write_int(chars, i, count) as _
            }
        } else {
            chars.len() as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn compress(chars: &mut Vec<char>) -> i32 {
        Self::compress(chars)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
