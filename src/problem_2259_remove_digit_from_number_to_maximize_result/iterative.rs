pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let digit = digit as u8;
        let mut iter = number.bytes().enumerate();
        let mut remove = 0;

        'outer: while let Some((mut i, mut left)) = iter.next() {
            while left == digit {
                remove = i;

                if let Some((j, right)) = iter.next() {
                    if left < right {
                        break 'outer;
                    }

                    i = j;
                    left = right;
                } else {
                    break 'outer;
                }
            }
        }

        let mut number = number;

        number.remove(remove);

        number
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_digit(number: String, digit: char) -> String {
        Self::remove_digit(number, digit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
