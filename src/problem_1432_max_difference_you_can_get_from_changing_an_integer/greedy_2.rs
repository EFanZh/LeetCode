pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn replace(iter: impl Iterator<Item = u8>, mut base: u32, source: u8, target: u8) -> u32 {
        for mut digit in iter {
            if digit == source {
                digit = target;
            }

            base = base * 10 + u32::from(digit);
        }

        base
    }

    fn find_and_replace(mut iter: impl Iterator<Item = u8>, mut base: u32, mut f: impl FnMut(u8) -> Option<u8>) -> u32 {
        for digit in iter.by_ref() {
            base *= 10;

            if let Some(replace_with) = f(digit) {
                base += u32::from(replace_with);

                return Self::replace(iter, base, digit, replace_with);
            }

            base += u32::from(digit);
        }

        base
    }

    #[allow(clippy::unnecessary_lazy_evaluations)] // Not supported by LeetCode.
    pub fn max_diff(num: i32) -> i32 {
        let mut num = num as u32;
        let mut digits = [0_u8; 10];
        let mut i = 10;

        while num != 0 {
            i -= 1;
            digits[i] = (num % 10) as _;
            num /= 10;
        }

        let mut iter = digits[i..].iter().copied();

        let (min, max) = match iter.next().unwrap() {
            1 => (
                Self::find_and_replace(iter.clone(), 1, |digit| (digit > 1).then(|| 0)),
                Self::replace(iter, 9, 1, 9),
            ),
            9 => (
                Self::replace(iter.clone(), 1, 9, 1),
                Self::find_and_replace(iter, 9, |digit| (digit < 9).then(|| 9)),
            ),
            first => (
                Self::replace(iter.clone(), 1, first, 1),
                Self::replace(iter, 9, first, 9),
            ),
        };

        (max - min) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_diff(num: i32) -> i32 {
        Self::max_diff(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
