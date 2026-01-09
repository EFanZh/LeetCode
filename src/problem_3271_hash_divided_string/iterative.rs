pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        let mut restart = k;
        let mut sum = b'a';

        s.retain_mut(|target| {
            sum += *target - b'a';

            if sum > b'z' {
                sum -= 26;
            }

            restart -= 1;

            if restart == 0 {
                *target = sum;
                restart = k;
                sum = b'a';

                true
            } else {
                false
            }
        });

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn string_hash(s: String, k: i32) -> String {
        Self::string_hash(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
