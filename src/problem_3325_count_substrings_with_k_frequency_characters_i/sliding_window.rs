pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let s = s.into_bytes();
        let mut result = 0;
        let mut start = 0;
        let mut countdowns = [k as u16 - 1; 26];

        for &c in &s {
            let countdown = &mut countdowns[usize::from(c) - usize::from(b'a')];

            if *countdown == 0 {
                loop {
                    let old = s[start];

                    start += 1;

                    if old == c {
                        break;
                    }

                    countdowns[usize::from(old) - usize::from(b'a')] += 1;
                }
            } else {
                *countdown -= 1;
            }

            result += start;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_substrings(s: String, k: i32) -> i32 {
        Self::number_of_substrings(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
