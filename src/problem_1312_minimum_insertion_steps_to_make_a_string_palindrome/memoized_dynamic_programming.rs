pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

struct Context<'a> {
    s: &'a [u8],
    cache: HashMap<(u16, u16), u32>,
}

impl Solution {
    fn helper(context: &mut Context, key: (u16, u16)) -> u32 {
        if let [head, .., tail] = context.s[usize::from(key.0)..usize::from(key.1)] {
            if let Some(&result) = context.cache.get(&key) {
                result
            } else {
                let result = if head == tail {
                    Self::helper(context, (key.0 + 1, key.1 - 1))
                } else {
                    u32::min(
                        Self::helper(context, (key.0, key.1 - 1)),
                        Self::helper(context, (key.0 + 1, key.1)),
                    ) + 1
                };

                context.cache.insert(key, result);

                result
            }
        } else {
            0
        }
    }

    pub fn min_insertions(s: String) -> i32 {
        Self::helper(
            &mut Context {
                s: s.as_bytes(),
                cache: HashMap::new(),
            },
            (0, s.len() as _),
        ) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_insertions(s: String) -> i32 {
        Self::min_insertions(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
