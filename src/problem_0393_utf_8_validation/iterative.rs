pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut i = 0;

        while let Some(first) = data.get(i) {
            match first {
                0..=127 => i += 1,
                192..=223 => {
                    if let Some([128..=191]) = data.get(i + 1..i + 2) {
                        i += 2;
                    } else {
                        return false;
                    }
                }
                224..=239 => {
                    if let Some([128..=191, 128..=191]) = data.get(i + 1..i + 3) {
                        i += 3;
                    } else {
                        return false;
                    }
                }
                240..=247 => {
                    if let Some([128..=191, 128..=191, 128..=191]) = data.get(i + 1..i + 4) {
                        i += 4;
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_utf8(data: Vec<i32>) -> bool {
        Self::valid_utf8(data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
