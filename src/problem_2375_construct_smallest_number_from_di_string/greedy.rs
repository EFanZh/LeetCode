pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let n = pattern.len();
        let mut result = String::with_capacity(n + 1);
        let mut prev = b'I';
        let mut length = 0;
        let mut c = b'1';

        for direction in pattern.bytes() {
            if direction == prev {
                length += 1;
            } else {
                if prev == b'I' {
                    result.extend((c - length..c).map(char::from));
                    length = 1;
                } else {
                    result.extend((c - length..=c).map(char::from).rev());
                    length = 0;
                }

                prev = direction;
            }

            c += 1;
        }

        drop(pattern);

        let iter = (c - length..=c).map(char::from);

        if prev == b'I' {
            result.extend(iter);
        } else {
            result.extend(iter.rev());
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_number(pattern: String) -> String {
        Self::smallest_number(pattern)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
