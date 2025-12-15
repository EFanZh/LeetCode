pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Context {
    buffer: String,
    result: Vec<String>,
    length: usize,
}

impl Context {
    fn emit(&mut self) {
        self.result.push(self.buffer.clone());
    }

    fn push<const C: char>(&mut self, f: impl FnOnce(&mut Self)) {
        self.buffer.push(C);
        self.length -= 1;

        f(self);

        self.length += 1;
        self.buffer.pop();
    }
}

impl Solution {
    fn dfs_0(context: &mut Context) {
        if context.length == 0 {
            context.emit();
        } else {
            context.push::<'1'>(Self::dfs_1);
        }
    }

    fn dfs_1(context: &mut Context) {
        if context.length == 0 {
            context.emit();
        } else {
            context.push::<'0'>(Self::dfs_0);
            context.push::<'1'>(Self::dfs_1);
        }
    }

    pub fn valid_strings(n: i32) -> Vec<String> {
        let n = n.cast_unsigned() as usize;

        let mut context = Context {
            buffer: String::with_capacity(n),
            result: Vec::new(),
            length: n,
        };

        Self::dfs_1(&mut context);

        context.result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_strings(n: i32) -> Vec<String> {
        Self::valid_strings(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
