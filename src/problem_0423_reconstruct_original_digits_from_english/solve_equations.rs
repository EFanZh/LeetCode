pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut letter_counts = [0; 26];

        for c in s.bytes() {
            letter_counts[usize::from(c - b'a')] += 1;
        }

        // ================================================
        //     e  f  g  h  i  n  o  r  s  t  u  v  w  x  z
        // ------------------------------------------------
        //  0  0  .  .  .  .  .  0  0  .  .  .  .  .  .  0
        //  1  1  .  .  .  .  1  1  .  .  .  .  .  .  .  .
        //  2  .  .  .  .  .  .  2  .  .  2  .  .  2  .  .
        //  3  3  .  .  3  .  .  .  3  .  3  .  .  .  .  .
        //  4  .  4  .  .  .  .  4  4  .  .  4  .  .  .  .
        //  5  5  5  .  .  5  .  .  .  .  .  .  5  .  .  .
        //  6  .  .  .  .  6  .  .  .  6  .  .  .  .  6  .
        //  7  7  .  .  .  .  7  .  .  7  .  .  7  .  .  .
        //  8  8  .  8  8  8  .  .  .  .  8  .  .  .  .  .
        //  9  9  .  .  .  9  9  .  .  .  .  .  .  .  .  .
        //           *                       *     *  *  *
        // ================================================

        let zero = letter_counts[usize::from(b'z' - b'a')];
        let two = letter_counts[usize::from(b'w' - b'a')];
        let four = letter_counts[usize::from(b'u' - b'a')];
        let six = letter_counts[usize::from(b'x' - b'a')];
        let eight = letter_counts[usize::from(b'g' - b'a')];
        let one = letter_counts[usize::from(b'o' - b'a')] - zero - two - four;
        let three = letter_counts[usize::from(b'h' - b'a')] - eight;
        let five = letter_counts[usize::from(b'f' - b'a')] - four;
        let seven = letter_counts[usize::from(b's' - b'a')] - six;
        let nine = (letter_counts[usize::from(b'n' - b'a')] - one - seven) / 2;

        let digit_counts = [zero, one, two, three, four, five, six, seven, eight, nine];
        let mut result = vec![0; digit_counts.iter().sum()];
        let mut slice = result.as_mut_slice();

        for (d, &count) in (b'0'..).zip(&digit_counts) {
            let (current, rest) = slice.split_at_mut(count);

            for target in current {
                *target = d;
            }

            slice = rest;
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn original_digits(s: String) -> String {
        Self::original_digits(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
