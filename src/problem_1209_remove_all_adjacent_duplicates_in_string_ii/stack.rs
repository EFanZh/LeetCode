pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let k = k as u32;
        let mut stack = Vec::new();

        for c in s.bytes() {
            if let Some((_, top_count)) = stack.last_mut().filter(|(top_c, _)| *top_c == c) {
                *top_count += 1;

                if *top_count == k {
                    stack.pop();
                }
            } else {
                stack.push((c, 1));
            }
        }

        let mut s = s.into_bytes();

        s.clear();

        for (c, count) in stack {
            for _ in 0..count {
                s.push(c);
            }
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_duplicates(s: String, k: i32) -> String {
        Self::remove_duplicates(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
