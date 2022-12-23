// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct CustomStack {
    buffer: Box<[(i32, i32)]>,
    length: usize,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            buffer: vec![(0, 0); max_size as _].into_boxed_slice(),
            length: 0,
        }
    }

    fn push(&mut self, x: i32) {
        if let Some(slot) = self.buffer.get_mut(self.length) {
            *slot = (x, 0);

            self.length += 1;
        }
    }

    fn pop(&mut self) -> i32 {
        if let Some(&top) = self.buffer.get(self.length.wrapping_sub(1)) {
            self.length -= 1;

            if let Some(new_top) = self.buffer.get_mut(self.length.wrapping_sub(1)) {
                new_top.1 += top.1;
            }

            top.0 + top.1
        } else {
            -1
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        if let Some((_, extra)) = self
            .buffer
            .get_mut((k as u32 as usize).min(self.length).wrapping_sub(1))
        {
            *extra += val;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::CustomStack for CustomStack {
    fn new(max_size: i32) -> Self {
        Self::new(max_size)
    }

    fn push(&mut self, x: i32) {
        self.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.pop()
    }

    fn increment(&mut self, k: i32, val: i32) {
        self.increment(k, val);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::CustomStack>();
    }
}
