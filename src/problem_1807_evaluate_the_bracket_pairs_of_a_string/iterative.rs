pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::convert::TryInto;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let s = s.as_bytes();
        let mut dictionary = HashMap::new();

        for knowledge in &knowledge {
            let [key, value]: &[_; 2] = knowledge.as_slice().try_into().ok().unwrap();

            dictionary.insert(key.as_bytes(), value.as_bytes());
        }

        let mut result = Vec::new();
        let mut i = 0;

        while let Some(&c) = s.get(i) {
            i += 1;

            if c == b'(' {
                let start = i;

                while s[i] != b')' {
                    i += 1;
                }

                result.extend(dictionary.get(&s[start..i]).copied().unwrap_or(b"?"));

                i += 1;
            } else {
                result.push(c);
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        Self::evaluate(s, knowledge)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
