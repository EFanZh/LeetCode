pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str;

struct Context {
    base: Vec<u8>,
    result: Vec<String>,
}

impl Context {
    fn add_result(&mut self) {
        self.result.push(str::from_utf8(&self.base).unwrap().to_string());
    }
}

impl Solution {
    fn helper(context: &mut Context, n: u32, k: u32) {
        if n == 0 {
            context.add_result();
        } else {
            context.base.push(b'0');
            Self::helper(context, n - 1, k);
            context.base.pop();

            let cost = context.base.len() as u32;

            if cost <= k {
                context.base.push(b'1');

                if n == 1 {
                    context.add_result();
                } else {
                    context.base.push(b'0');
                    Self::helper(context, n - 2, k - cost);
                    context.base.pop();
                }

                context.base.pop();
            }
        }
    }

    pub fn generate_valid_strings(n: i32, k: i32) -> Vec<String> {
        let n = n.cast_unsigned();

        let mut context = Context {
            base: Vec::with_capacity(n as _),
            result: Vec::new(),
        };

        Self::helper(&mut context, n, k.cast_unsigned());

        context.result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn generate_valid_strings(n: i32, k: i32) -> Vec<String> {
        Self::generate_valid_strings(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
