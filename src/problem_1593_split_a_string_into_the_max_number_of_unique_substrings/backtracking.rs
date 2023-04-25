pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

struct Context {
    used: HashSet<[u8; 16]>,
    result: usize,
}

impl Solution {
    fn helper(context: &mut Context, s: &[u8]) {
        if s.is_empty() {
            context.result = context.result.max(context.used.len());
        } else {
            let used_length = context.used.len();
            let n = s.len();
            let mut i = 0;
            let mut buffer = [0; 16];

            while i < n - context.result.saturating_sub(used_length) {
                buffer[i] = s[i];

                if context.used.insert(buffer) {
                    Self::helper(context, &s[i + 1..]);

                    context.used.remove(&buffer);
                }

                i += 1;
            }
        }
    }

    pub fn max_unique_split(s: String) -> i32 {
        let mut context = Context {
            used: HashSet::new(),
            result: 0,
        };

        Self::helper(&mut context, s.as_bytes());

        context.result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_unique_split(s: String) -> i32 {
        Self::max_unique_split(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
