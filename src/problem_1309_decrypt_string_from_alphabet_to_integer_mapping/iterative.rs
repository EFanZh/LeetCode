pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut result = Vec::with_capacity(s.len());
        let mut iter = s.bytes();

        'outer: while let Some(mut first) = iter.next() {
            if let Some(mut second) = iter.next() {
                loop {
                    if let Some(third) = iter.next() {
                        if third == b'#' {
                            result.push(b'j' + (first - b'0') * 10 + (second - b'0') - 10);

                            break;
                        }

                        result.push(b'a' + (first - b'1'));

                        first = second;
                        second = third;
                    } else {
                        result.push(b'a' + (first - b'1'));
                        result.push(b'a' + (second - b'1'));

                        break 'outer;
                    }
                }
            } else {
                result.push(b'a' + (first - b'1'));

                break;
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn freq_alphabets(s: String) -> String {
        Self::freq_alphabets(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
