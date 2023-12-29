pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut iter = s.split_whitespace().filter_map(|word| word.parse::<u8>().ok());
        let mut prev = iter.next().unwrap();

        for value in iter {
            if value <= prev {
                return false;
            }

            prev = value;
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn are_numbers_ascending(s: String) -> bool {
        Self::are_numbers_ascending(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
