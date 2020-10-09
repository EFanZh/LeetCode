pub struct Solution;

use std::vec::IntoIter;

impl Solution {
    // \s*[+-]?(\d+\.?|\.\d)\d*(e[+-]?\d+)?\s*

    fn start(mut iter: IntoIter<u8>) -> bool {
        iter.next().map_or(false, |c| match c {
            b' ' => Self::start(iter),
            b'+' | b'-' => Self::number_without_sign(iter),
            b'.' => Self::fractional_part(iter),
            b'0'..=b'9' => Self::number_with_optional_fractional_part(iter),
            _ => false,
        })
    }

    // (\d+\.?|\.\d)\d*(e[+-]?\d+)?\s*

    fn number_without_sign(mut iter: IntoIter<u8>) -> bool {
        iter.next().map_or(false, |c| match c {
            b'.' => Self::fractional_part(iter),
            b'0'..=b'9' => Self::number_with_optional_fractional_part(iter),
            _ => false,
        })
    }

    // \d+(e[+-]?\d+)?\s*

    fn fractional_part(mut iter: IntoIter<u8>) -> bool {
        iter.next().map_or(false, |c| match c {
            b'0'..=b'9' => Self::optional_fractional_part(iter),
            _ => false,
        })
    }

    // \d*\.?\d*(e[+-]?\d+)?\s*

    fn number_with_optional_fractional_part(mut iter: IntoIter<u8>) -> bool {
        iter.next().map_or(true, |c| match c {
            b'0'..=b'9' => Self::number_with_optional_fractional_part(iter),
            b'.' => Self::optional_fractional_part(iter),
            b'e' => Self::exponential_part(iter),
            b' ' => Self::trailing_space(iter),
            _ => false,
        })
    }

    // \d*(e[+-]?\d+)?\s*

    fn optional_fractional_part(mut iter: IntoIter<u8>) -> bool {
        iter.next().map_or(true, |c| match c {
            b'0'..=b'9' => Self::optional_fractional_part(iter),
            b'e' => Self::exponential_part(iter),
            b' ' => Self::trailing_space(iter),
            _ => false,
        })
    }

    // [+-]?\d+\s*

    fn exponential_part(mut iter: IntoIter<u8>) -> bool {
        iter.next().map_or(false, |c| match c {
            b'+' | b'-' => Self::exponent(iter),
            b'0'..=b'9' => Self::optional_exponent(iter),
            _ => false,
        })
    }

    // \d+\s*

    fn exponent(mut iter: IntoIter<u8>) -> bool {
        iter.next().map_or(false, |c| match c {
            b'0'..=b'9' => Self::optional_exponent(iter),
            _ => false,
        })
    }

    // \d*\s*

    fn optional_exponent(mut iter: IntoIter<u8>) -> bool {
        iter.next().map_or(true, |c| match c {
            b'0'..=b'9' => Self::optional_exponent(iter),
            b' ' => Self::trailing_space(iter),
            _ => false,
        })
    }

    // \s*

    fn trailing_space(mut iter: IntoIter<u8>) -> bool {
        iter.next().map_or(true, |c| match c {
            b' ' => Self::trailing_space(iter),
            _ => false,
        })
    }

    pub fn is_number(s: String) -> bool {
        // \s*[+-]?(\d+\.?|\.\d)\d*(e[+-]?\d+)?\s*
        //
        // https://cyberzhg.github.io/toolbox/min_dfa?regex=cyoocHxtKT8oZCsuP3wuZClkKihlKHB8bSk/ZCspP3Mq.

        Self::start(s.into_bytes().into_iter())
    }
}

impl super::Solution for Solution {
    fn is_number(s: String) -> bool {
        Self::is_number(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
