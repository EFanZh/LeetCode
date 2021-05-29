pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() <= s2.len() {
            let s1 = s1.into_bytes();
            let s2 = s2.into_bytes();
            let mut expected = [0_u16; 26];

            for c in &s1 {
                expected[usize::from(c - b'a')] += 1;
            }

            let mut window = [0_u16; 26];
            let (left, right) = s2.split_at(s1.len());

            for c in left {
                window[usize::from(c - b'a')] += 1;
            }

            let mut diff = window.iter().zip(&expected).filter(|(x, y)| x != y).count() as u8;

            if diff == 0 {
                return true;
            }

            for (old, new) in s2.iter().zip(right).filter(|(old, new)| old != new) {
                let old_count = &mut window[usize::from(old - b'a')];
                let saved_old_count = *old_count;
                let expected_old_count = expected[usize::from(old - b'a')];

                *old_count -= 1;

                if saved_old_count == expected_old_count {
                    diff += 1;
                } else if *old_count == expected_old_count {
                    diff -= 1;
                }

                let new_count = &mut window[usize::from(new - b'a')];
                let saved_new_count = *new_count;
                let expected_new_count = expected[usize::from(new - b'a')];

                *new_count += 1;

                if saved_new_count == expected_new_count {
                    diff += 1;
                } else if *new_count == expected_new_count {
                    diff -= 1;
                }

                if diff == 0 {
                    return true;
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_inclusion(s1: String, s2: String) -> bool {
        Self::check_inclusion(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
