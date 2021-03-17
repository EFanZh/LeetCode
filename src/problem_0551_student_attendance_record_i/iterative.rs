pub struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent = false;
        let mut late = 0;

        for c in s.bytes() {
            match c {
                b'A' => {
                    if absent {
                        return false;
                    }

                    absent = true;
                    late = 0;
                }
                b'L' => {
                    if late == 2 {
                        return false;
                    }

                    late += 1;
                }
                _ => late = 0,
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn check_record(s: String) -> bool {
        Self::check_record(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
