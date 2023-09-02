pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut s = s;
        let bytes = s.as_bytes();
        let mut i = 0;
        let mut k = k;

        while let Some(&c) = bytes.get(i) {
            if c == b' ' {
                k -= 1;

                if k == 0 {
                    s.truncate(i);

                    break;
                }
            }

            i += 1;
        }

        s
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn truncate_sentence(s: String, k: i32) -> String {
        Self::truncate_sentence(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
