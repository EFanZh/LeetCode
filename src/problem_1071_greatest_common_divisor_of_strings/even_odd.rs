pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut s1 = str1.as_str();
        let mut s2 = str2.as_str();

        loop {
            if s2.len() < s1.len() {
                mem::swap(&mut s1, &mut s2);
            }

            if s1.is_empty() {
                break;
            }

            if let Some(new_s2) = s2.strip_prefix(s1) {
                s2 = new_s2;
            } else {
                s2 = "";

                break;
            }
        }

        let length = s2.len();
        let mut result = str1;

        result.truncate(length);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn gcd_of_strings(str1: String, str2: String) -> String {
        Self::gcd_of_strings(str1, str2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
