pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(mut iter: impl Iterator<Item = u8>) -> u8 {
        let mut current = 0;
        let mut max = 0;

        loop {
            loop {
                if let Some(c) = iter.next() {
                    if c == current {
                        break;
                    }

                    current = c;
                } else {
                    return max;
                }
            }

            loop {
                if let Some(c) = iter.next() {
                    if c == current {
                        max = max.max(c);
                    } else {
                        current = c;

                        break;
                    }
                } else {
                    return max;
                }
            }
        }
    }

    pub fn largest_good_integer(num: String) -> String {
        let best = Self::helper(num.bytes());
        let mut result = num;

        result.clear();

        if best != 0 {
            result.extend([char::from(best), char::from(best), char::from(best)]);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_good_integer(num: String) -> String {
        Self::largest_good_integer(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
