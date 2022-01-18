// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct RLEIterator {
    encoding: Vec<i32>,
    i: usize,
}

impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self { encoding, i: 0 }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut n = n;

        while let Some(count) = self.encoding.get_mut(self.i) {
            if n as u32 <= *count as u32 {
                *count -= n;

                return self.encoding[self.i + 1];
            }

            n -= *count;
            self.i += 2;
        }

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::RLEIterator for RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self::new(encoding)
    }

    fn next(&mut self, n: i32) -> i32 {
        self.next(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::RLEIterator>();
    }
}
