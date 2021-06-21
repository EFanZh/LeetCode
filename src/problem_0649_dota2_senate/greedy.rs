pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut senate = senate.into_bytes();
        let mut temp = Vec::with_capacity(senate.len());
        let mut count = 0;

        loop {
            for &s in &senate {
                if s == b'R' {
                    count += 1;

                    if count > 0 {
                        temp.push(s);
                    }
                } else {
                    count -= 1;

                    if count < 0 {
                        temp.push(s);
                    }
                }
            }

            if temp.len() == senate.len() {
                break;
            }

            mem::swap(&mut senate, &mut temp);
            temp.clear();
        }

        String::from(if senate.starts_with(b"R") { "Radiant" } else { "Dire" })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn predict_party_victory(senate: String) -> String {
        Self::predict_party_victory(senate)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
