pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut iter = s.into_bytes().into_iter();

        while let Some(c) = iter.next_back() {
            if c != b' ' {
                let mut result = 1;

                while let Some(c) = iter.next_back() {
                    if c == b' ' {
                        break;
                    } else {
                        result += 1;
                    }
                }

                return result;
            }
        }

        0
    }
}

impl super::Solution for Solution {
    fn length_of_last_word(s: String) -> i32 {
        Self::length_of_last_word(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
