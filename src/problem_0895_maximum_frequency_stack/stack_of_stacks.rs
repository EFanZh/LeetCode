// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

pub struct FreqStack {
    frequencies: HashMap<i32, usize>,
    stacks: Vec<Vec<i32>>,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            frequencies: HashMap::new(),
            stacks: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let frequency = *self.frequencies.entry(val).and_modify(|count| *count += 1).or_insert(1);

        if let Some(stack) = self.stacks.get_mut(frequency - 1) {
            stack.push(val);
        } else {
            self.stacks.push(vec![val]);
        }
    }

    fn pop(&mut self) -> i32 {
        let frequency_is_one = self.stacks.len() == 1;
        let stack = self.stacks.last_mut().unwrap();
        let result = stack.pop().unwrap();

        if stack.is_empty() {
            self.stacks.pop();
        }

        if frequency_is_one {
            self.frequencies.remove(&result);
        } else {
            *self.frequencies.get_mut(&result).unwrap() -= 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::FreqStack for FreqStack {
    fn new() -> Self {
        Self::new()
    }

    fn push(&mut self, val: i32) {
        self.push(val);
    }

    fn pop(&mut self) -> i32 {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::FreqStack>();
    }
}
